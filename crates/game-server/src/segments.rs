//! Guest data-segment blob.
//!
//! The real module carries 4,077 passive data segments plus a 953 KB inlined
//! static-init region (`__wasm_init_memory` -> `__wasm_call_ctors`). The host
//! binary cannot ship that as a Rust literal, so the build tooling exports it
//! to a compact blob that this module parses and the runtime replays:
//!
//! ```text
//! offset  size  field
//! 0       4     magic "WSEG"
//! 4       4     version, u32 LE (must be 1)
//! 8       4     segment count, u32 LE
//! 12      ...   count x entry
//!
//! entry: u64 target_offset, u64 length, u32 flags (0 active, 1 passive),
//!        u32 reserved, then `length` bytes of payload
//! ```
//!
//! A missing blob is not an error: the server boots with all-zero memory and
//! says so, which is exactly what the empty-data-dir smoke test needs.

use std::fmt;
use std::path::{Path, PathBuf};

use crate::log_info;
use crate::mem::LinearMemory;

pub const MAGIC: [u8; 4] = *b"WSEG";
pub const VERSION: u32 = 1;
const HEADER_BYTES: usize = 12;
const ENTRY_BYTES: usize = 24;
const MAX_SEGMENTS: usize = 1 << 20;

#[derive(Debug)]
pub enum SegError {
    Io(PathBuf, std::io::Error),
    Truncated { at: usize, need: usize, have: usize },
    BadMagic([u8; 4]),
    BadVersion(u32),
    TooManySegments(u32),
    Overlap { offset: u64, len: u64 },
}

impl fmt::Display for SegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegError::Io(path, e) => write!(f, "reading {}: {e}", path.display()),
            SegError::Truncated { at, need, have } => write!(
                f,
                "segment blob truncated at byte {at}: need {need}, have {have}"
            ),
            SegError::BadMagic(m) => write!(f, "segment blob magic is {:?}, expected \"WSEG\"", m),
            SegError::BadVersion(v) => write!(f, "segment blob version {v} unsupported (want {VERSION})"),
            SegError::TooManySegments(n) => write!(f, "segment blob declares {n} segments (max {MAX_SEGMENTS})"),
            SegError::Overlap { offset, len } => {
                write!(f, "segment [{offset:#x}, +{len:#x}) is not one-page aligned/ordered")
            }
        }
    }
}

impl std::error::Error for SegError {}

#[derive(Debug, Clone)]
pub struct Segment {
    pub offset: u64,
    pub passive: bool,
    pub data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct SegmentBlob {
    pub segments: Vec<Segment>,
    pub path: Option<PathBuf>,
}

impl SegmentBlob {
    pub fn total_bytes(&self) -> u64 {
        self.segments.iter().map(|s| s.data.len() as u64).sum()
    }

    pub fn highest_offset(&self) -> u64 {
        self.segments
            .iter()
            .map(|s| s.offset.saturating_add(s.data.len() as u64))
            .max()
            .unwrap_or(0)
    }

    /// Serialise in the on-disk format (used by tests and the exporter check).
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + self.segments.len() * ENTRY_BYTES);
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&VERSION.to_le_bytes());
        out.extend_from_slice(&(self.segments.len() as u32).to_le_bytes());
        for seg in &self.segments {
            out.extend_from_slice(&seg.offset.to_le_bytes());
            out.extend_from_slice(&(seg.data.len() as u64).to_le_bytes());
            out.extend_from_slice(&(u32::from(seg.passive)).to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&seg.data);
        }
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<SegmentBlob, SegError> {
        if bytes.len() < HEADER_BYTES {
            return Err(SegError::Truncated { at: 0, need: HEADER_BYTES, have: bytes.len() });
        }
        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[0..4]);
        if magic != MAGIC {
            return Err(SegError::BadMagic(magic));
        }
        let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap_or([0; 4]));
        if version != VERSION {
            return Err(SegError::BadVersion(version));
        }
        let count = u32::from_le_bytes(bytes[8..12].try_into().unwrap_or([0; 4]));
        if count as usize > MAX_SEGMENTS {
            return Err(SegError::TooManySegments(count));
        }

        let mut cursor = HEADER_BYTES;
        let mut segments = Vec::with_capacity(count as usize);
        for index in 0..count as usize {
            if bytes.len() < cursor + ENTRY_BYTES {
                return Err(SegError::Truncated {
                    at: cursor,
                    need: ENTRY_BYTES,
                    have: bytes.len().saturating_sub(cursor),
                });
            }
            let offset = u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap_or([0; 8]));
            let len = u64::from_le_bytes(bytes[cursor + 8..cursor + 16].try_into().unwrap_or([0; 8]));
            let flags = u32::from_le_bytes(bytes[cursor + 16..cursor + 20].try_into().unwrap_or([0; 4]));
            cursor += ENTRY_BYTES;
            let len_usize = usize::try_from(len).map_err(|_| SegError::Truncated {
                at: cursor,
                need: usize::MAX,
                have: bytes.len().saturating_sub(cursor),
            })?;
            if bytes.len() < cursor + len_usize {
                return Err(SegError::Truncated {
                    at: cursor,
                    need: len_usize,
                    have: bytes.len().saturating_sub(cursor),
                });
            }
            if offset.checked_add(len).is_none() {
                return Err(SegError::Overlap { offset, len });
            }
            segments.push(Segment {
                offset,
                passive: flags != 0,
                data: bytes[cursor..cursor + len_usize].to_vec(),
            });
            cursor += len_usize;
            let _ = index;
        }
        Ok(SegmentBlob { segments, path: None })
    }
}

/// Candidate locations inside the data dir, in priority order.
fn candidates(dir: &Path) -> [PathBuf; 2] {
    [dir.join("memory").join("segments.bin"), dir.join("segments.bin")]
}

/// Load the blob if present. `Ok(None)` means "no blob", which is not fatal.
pub fn load_from_dir(dir: &Path) -> Result<Option<SegmentBlob>, SegError> {
    for path in candidates(dir) {
        match std::fs::read(&path) {
            Ok(bytes) => {
                let mut blob = SegmentBlob::decode(&bytes)?;
                log_info!(
                    "segments",
                    "loaded {} segments ({} bytes) from {}",
                    blob.segments.len(),
                    blob.total_bytes(),
                    path.display()
                );
                blob.path = Some(path);
                return Ok(Some(blob));
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(SegError::Io(path, e)),
        }
    }
    Ok(None)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InstallReport {
    pub segments: usize,
    pub passive: usize,
    pub bytes: u64,
}

/// Replay the blob into linear memory (`__wasm_init_memory`).
pub fn install(mem: &LinearMemory, blob: &SegmentBlob) -> Result<InstallReport, SegError> {
    let mut report = InstallReport::default();
    for seg in &blob.segments {
        if mem.write(seg.offset, &seg.data).is_err() {
            return Err(SegError::Overlap { offset: seg.offset, len: seg.data.len() as u64 });
        }
        report.segments += 1;
        report.bytes += seg.data.len() as u64;
        if seg.passive {
            report.passive += 1;
        }
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SegmentBlob {
        SegmentBlob {
            segments: vec![
                Segment { offset: 0x1000, passive: false, data: vec![1, 2, 3, 4] },
                Segment { offset: 0x8000, passive: true, data: vec![9; 300] },
            ],
            path: None,
        }
    }

    #[test]
    fn roundtrip_preserves_segments() {
        let blob = sample();
        let decoded = SegmentBlob::decode(&blob.encode()).unwrap();
        assert_eq!(decoded.segments.len(), 2);
        assert_eq!(decoded.segments[0].offset, 0x1000);
        assert_eq!(decoded.segments[1].data.len(), 300);
        assert!(decoded.segments[1].passive);
        assert_eq!(decoded.total_bytes(), 304);
        assert_eq!(decoded.highest_offset(), 0x8000 + 300);
    }

    #[test]
    fn rejects_bad_magic_and_version() {
        let mut bytes = sample().encode();
        bytes[0] = b'X';
        assert!(matches!(SegmentBlob::decode(&bytes), Err(SegError::BadMagic(_))));

        let mut bytes = sample().encode();
        bytes[4] = 9;
        assert!(matches!(SegmentBlob::decode(&bytes), Err(SegError::BadVersion(9))));
    }

    #[test]
    fn rejects_truncation() {
        let bytes = sample().encode();
        for cut in [3usize, 12, 20, bytes.len() - 1] {
            assert!(
                SegmentBlob::decode(&bytes[..cut]).is_err(),
                "truncation at {cut} should fail"
            );
        }
    }

    #[test]
    fn missing_blob_is_not_an_error() {
        let dir = std::env::temp_dir().join(format!("gs-seg-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        assert!(load_from_dir(&dir).unwrap().is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn install_writes_payload() {
        let mem = LinearMemory::reserve(1024 * 1024).unwrap();
        let report = install(&mem, &sample()).unwrap();
        assert_eq!(report.segments, 2);
        assert_eq!(report.passive, 1);
        assert_eq!(mem.read(0x1000, 4).unwrap(), vec![1, 2, 3, 4]);
        assert_eq!(mem.read(0x8000, 2).unwrap(), vec![9, 9]);
    }
}