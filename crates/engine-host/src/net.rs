//! HTTP range reads for assets that are not staged locally.
//!
//! `io_worker.js` used `fetch()` with a `Range` header for the streaming path
//! (`env.wasm_httpfs_pread` against the CDN). The server build speaks the same
//! protocol over `std::net::TcpStream`: `GET` + `Range: bytes=a-b`, one request
//! per read, connection closed by the server.
//!
//! Deliberately out of scope: TLS. A server deployment that needs `https://`
//! should front the CDN with a local cache or terminate TLS in front of this
//! host; [`HttpUrl::parse`] rejects non-`http` schemes instead of silently
//! downgrading.

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::host::{HostError, HostResult};

/// `env.wasm_httpfs_pread` in remote mode.
pub trait HttpRangeClient: Send + Sync {
    /// Fetch `len` bytes starting at `start`. Returns a short vector when the
    /// object ends before `start + len`.
    fn get_range(&self, url: &str, start: u64, len: u32) -> HostResult<Vec<u8>>;
}

/// The subset of a URL this client understands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpUrl {
    pub host: String,
    pub port: u16,
    /// Origin-form path including the query string.
    pub path: String,
}

impl HttpUrl {
    pub fn parse(raw: &str) -> HostResult<Self> {
        let url = raw.trim();
        let rest = url.strip_prefix("http://").ok_or_else(|| {
            if url.starts_with("https://") {
                HostError::Unsupported("https needs a TLS stack; use an http mirror")
            } else {
                HostError::InvalidArgument(format!("not an http url: {raw}"))
            }
        })?;
        let (authority, path) = match rest.find('/') {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, "/"),
        };
        let (host, port) = match authority.rsplit_once(':') {
            Some((h, p)) if !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()) => {
                let port: u16 = p
                    .parse()
                    .map_err(|_| HostError::InvalidArgument(format!("bad port in {raw}")))?;
                (h, port)
            }
            _ => (authority, 80),
        };
        if host.is_empty() || host.contains('@') {
            return Err(HostError::InvalidArgument(format!("bad authority in {raw}")));
        }
        Ok(Self {
            host: host.to_owned(),
            port,
            path: path.to_owned(),
        })
    }

    fn host_header(&self) -> String {
        if self.port == 80 {
            self.host.clone()
        } else {
            format!("{}:{}", self.host, self.port)
        }
    }
}

/// `std::net`-backed range reader. Holds only configuration, so it is trivially
/// `Send + Sync` and can be shared by every streamer worker.
#[derive(Clone, Debug)]
pub struct TcpHttpRangeClient {
    timeout: Duration,
}

impl TcpHttpRangeClient {
    pub fn new(timeout: Duration) -> HostResult<Self> {
        if timeout.is_zero() {
            return Err(HostError::InvalidArgument(
                "http timeout must be non-zero".into(),
            ));
        }
        Ok(Self { timeout })
    }

    fn request(&self, url: &HttpUrl, start: u64, len: u32) -> HostResult<(u16, Vec<u8>)> {
        let addr = (url.host.as_str(), url.port)
            .to_socket_addrs()
            .map_err(HostError::Io)?
            .next()
            .ok_or_else(|| HostError::NotFound(format!("{}:{}", url.host, url.port)))?;
        let mut stream =
            TcpStream::connect_timeout(&addr, self.timeout).map_err(HostError::Io)?;
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;

        let last = start.saturating_add(u64::from(len).saturating_sub(1));
        let request = format!(
            "GET {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Range: bytes={start}-{last}\r\n\
             User-Agent: engine-host/{}\r\n\
             Accept-Encoding: identity\r\n\
             Connection: close\r\n\r\n",
            url.path,
            url.host_header(),
            env!("CARGO_PKG_VERSION")
        );
        stream.write_all(request.as_bytes())?;
        stream.flush()?;

        let mut raw = Vec::new();
        stream.read_to_end(&mut raw)?;
        parse_response(&raw)
    }
}

impl HttpRangeClient for TcpHttpRangeClient {
    fn get_range(&self, url: &str, start: u64, len: u32) -> HostResult<Vec<u8>> {
        let parsed = HttpUrl::parse(url)?;
        let (status, body) = self.request(&parsed, start, len)?;
        match status {
            206 => Ok(body),
            // Server ignored `Range` and sent the whole object: slice locally
            // rather than failing — some CDNs do this for tiny files.
            200 => {
                let begin = usize::try_from(start).map_err(|_| {
                    HostError::OutOfRange("range start exceeds address space".into())
                })?;
                if begin >= body.len() {
                    return Ok(Vec::new());
                }
                let end = (begin + len as usize).min(body.len());
                Ok(body[begin..end].to_vec())
            }
            416 => Err(HostError::OutOfRange(format!(
                "{url}: range {start}+{len} not satisfiable"
            ))),
            other => Err(HostError::Protocol(format!("unexpected http status {other}"))),
        }
    }
}

/// Split an HTTP/1.1 response into status code and body.
fn parse_response(raw: &[u8]) -> HostResult<(u16, Vec<u8>)> {
    let split = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| HostError::Protocol("response has no header terminator".into()))?;
    let head = std::str::from_utf8(&raw[..split])
        .map_err(|_| HostError::Protocol("response headers are not utf-8".into()))?;
    let mut lines = head.split("\r\n");
    let status_line = lines
        .next()
        .ok_or_else(|| HostError::Protocol("empty response".into()))?;
    let mut parts = status_line.split_whitespace();
    let version = parts.next().unwrap_or_default();
    if !version.starts_with("HTTP/1.") {
        return Err(HostError::Protocol(format!("bad status line {status_line:?}")));
    }
    let status: u16 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| HostError::Protocol(format!("bad status line {status_line:?}")))?;

    // Chunked encoding is not negotiated (`Accept-Encoding: identity`, and we
    // never ask for HTTP/1.1 chunking), so anything chunked is a protocol
    // error rather than something to silently mangle.
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("transfer-encoding")
                && value.trim().to_ascii_lowercase().contains("chunked")
            {
                return Err(HostError::Protocol(
                    "chunked responses are not supported".into(),
                ));
            }
        }
    }
    Ok((status, raw[split + 4..].to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn url_parsing() {
        let url = HttpUrl::parse("http://cdn.local:8080/a/b.bin?x=1").expect("parse");
        assert_eq!(url.host, "cdn.local");
        assert_eq!(url.port, 8080);
        assert_eq!(url.path, "/a/b.bin?x=1");
        assert_eq!(url.host_header(), "cdn.local:8080");

        let url = HttpUrl::parse("http://example.com").expect("parse");
        assert_eq!(url.path, "/");
        assert_eq!(url.host_header(), "example.com");

        assert!(matches!(
            HttpUrl::parse("https://example.com/x"),
            Err(HostError::Unsupported(_))
        ));
        assert!(HttpUrl::parse("file:///etc/passwd").is_err());
        assert!(HttpUrl::parse("http:///nope").is_err());
    }

    #[test]
    fn response_parsing_rejects_garbage() {
        assert!(matches!(
            parse_response(b"nonsense"),
            Err(HostError::Protocol(_))
        ));
        assert!(matches!(
            parse_response(b"HTTP/1.1 abc\r\n\r\n"),
            Err(HostError::Protocol(_))
        ));
        let chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n";
        assert!(matches!(
            parse_response(chunked),
            Err(HostError::Protocol(_))
        ));
        let (status, body) = parse_response(b"HTTP/1.1 206 Partial\r\nX: y\r\n\r\nabcd").expect("ok");
        assert_eq!(status, 206);
        assert_eq!(body, b"abcd");
    }

    /// One-shot HTTP server: serves `body`, honours `Range` with a 206.
    fn serve_once(listener: TcpListener, body: &'static [u8]) -> std::thread::JoinHandle<String> {
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap_or(0);
            let request = String::from_utf8_lossy(&buf[..n]).to_string();
            let range = request
                .lines()
                .find_map(|l| l.strip_prefix("Range: bytes="))
                .map(|v| v.trim().to_owned());
            let response = match range.and_then(|r| {
                let (a, b) = r.split_once('-')?;
                Some((a.parse::<usize>().ok()?, b.parse::<usize>().ok()?))
            }) {
                Some((a, b)) => {
                    let end = (b + 1).min(body.len());
                    let slice = body.get(a..end).unwrap_or(&[]);
                    let mut out = format!(
                        "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        slice.len()
                    )
                    .into_bytes();
                    out.extend_from_slice(slice);
                    out
                }
                None => {
                    let mut out =
                        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len())
                            .into_bytes();
                    out.extend_from_slice(body);
                    out
                }
            };
            let _ = stream.write_all(&response);
            request
        })
    }

    #[test]
    fn ranged_get_round_trips_over_tcp() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = serve_once(listener, b"0123456789");
        let client = TcpHttpRangeClient::new(Duration::from_secs(5)).expect("client");
        let url = format!("http://127.0.0.1:{}/assets/chunk.bin", addr.port());

        let bytes = client.get_range(&url, 3, 4).expect("range");
        assert_eq!(bytes, b"3456");

        let request = server.join().expect("server");
        assert!(request.starts_with("GET /assets/chunk.bin HTTP/1.1"), "{request}");
        assert!(request.contains("Range: bytes=3-6"), "{request}");
    }

    #[test]
    fn ignored_range_header_falls_back_to_slicing() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = std::thread::spawn(move || {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().expect("accept");
                let mut buf = [0u8; 512];
                let _ = stream.read(&mut buf);
                stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nabcde").expect("response");
            }
        });
        let client = TcpHttpRangeClient::new(Duration::from_secs(5)).expect("client");
        let url = format!("http://127.0.0.1:{}/x", addr.port());
        assert_eq!(client.get_range(&url, 1, 3).expect("range"), b"bcd");
        assert_eq!(client.get_range(&url, 9, 3).expect("past eof"), b"");
        server.join().expect("server");
    }

    #[test]
    fn zero_timeout_is_rejected() {
        assert!(TcpHttpRangeClient::new(Duration::ZERO).is_err());
    }
}
