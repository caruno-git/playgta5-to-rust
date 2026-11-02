//! File-backed storage: the asset tree and the userdata blob store.
//!
//! Both are `std::fs` today, but the engine only ever sees the [`AssetStore`]
//! and [`userdata::UserdataStore`] traits, so a packed archive or a remote
//! store can be substituted without touching generated code.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::host::{HostError, HostResult};

/// Numeric handle the engine uses for an asset. This is the value
/// `env.wasm_httpfs_open` returns (`-1`/`u32::MAX` meant "missing").
pub type AssetId = u32;

/// Sentinel returned by `env.wasm_httpfs_open` on failure.
pub const NO_ASSET: AssetId = u32::MAX;

/// One line of the asset manifest shared with `io_worker.js`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetEntry {
    pub id: AssetId,
    /// Path relative to the asset root, always `/`-separated in the manifest.
    pub path: String,
    pub size: u64,
}

/// The manifest the engine and the host agree on.
///
/// Wire format (tab separated, one record per line, `#` comments):
///
/// ```text
/// # id<TAB>size<TAB>path
/// 0	1024	common/data/handling.meta
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AssetManifest {
    entries: Vec<AssetEntry>,
    by_path: BTreeMap<String, AssetId>,
}

impl AssetManifest {
    pub fn parse_engine_json(text: &str) -> HostResult<Self> {
        let value: serde_json::Value = serde_json::from_str(text).map_err(|e| HostError::Manifest(e.to_string()))?;
        let files = value.get("files").and_then(|v| v.as_array()).ok_or_else(|| HostError::Manifest("missing files array".into()))?;
        let mut entries = Vec::with_capacity(files.len());
        let mut by_path = BTreeMap::new();
        for (index, file) in files.iter().enumerate() {
            let path = file.get(0).and_then(|v| v.as_str()).ok_or_else(|| HostError::Manifest(format!("invalid path at {index}")))?;
            let size = file.get(1).and_then(|v| v.as_u64()).ok_or_else(|| HostError::Manifest(format!("invalid size at {index}")))?;
            let id = u32::try_from(index).map_err(|_| HostError::Manifest("too many assets".into()))?;
            if path.is_empty() || by_path.insert(path.to_owned(), id).is_some() { return Err(HostError::Manifest(format!("duplicate or empty path at {index}"))); }
            entries.push(AssetEntry { id, path: path.to_owned(), size });
        }
        Ok(Self { entries, by_path })
    }

    pub fn to_engine_json(&self) -> HostResult<String> {
        let mut entries: Vec<_> = self.entries.iter().collect();
        entries.sort_by_key(|e| e.id);
        if entries.iter().enumerate().any(|(i, e)| e.id as usize != i) { return Err(HostError::Manifest("engine asset IDs must be contiguous".into())); }
        let files: Vec<_> = entries.iter().map(|e| (&e.path, e.size)).collect();
        serde_json::to_string(&serde_json::json!({"version":"rust-port-1", "files":files})).map_err(|e| HostError::Manifest(e.to_string()))
    }

    pub fn parse(text: &str) -> HostResult<Self> {
        let mut entries = Vec::new();
        let mut by_path = BTreeMap::new();
        for (lineno, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches('\r');
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut fields = line.splitn(3, '\t');
            let id = fields.next().unwrap_or_default();
            let size = fields.next().unwrap_or_default();
            let path = fields.next().unwrap_or_default();
            if path.is_empty() {
                return Err(HostError::Manifest(format!(
                    "line {}: expected id<TAB>size<TAB>path",
                    lineno + 1
                )));
            }
            let id: AssetId = id.trim().parse().map_err(|_| {
                HostError::Manifest(format!("line {}: bad id {id:?}", lineno + 1))
            })?;
            let size: u64 = size.trim().parse().map_err(|_| {
                HostError::Manifest(format!("line {}: bad size {size:?}", lineno + 1))
            })?;
            if by_path.contains_key(path) {
                return Err(HostError::Manifest(format!(
                    "line {}: duplicate path {path}",
                    lineno + 1
                )));
            }
            by_path.insert(path.to_owned(), id);
            entries.push(AssetEntry {
                id,
                path: path.to_owned(),
                size,
            });
        }
        Ok(Self { entries, by_path })
    }

    pub fn to_text(&self) -> String {
        let mut out = String::from("# id\tsize\tpath\n");
        for e in &self.entries {
            out.push_str(&format!("{}\t{}\t{}\n", e.id, e.size, e.path));
        }
        out
    }

    pub fn lookup(&self, path: &str) -> Option<AssetId> {
        self.by_path.get(path).copied()
    }

    pub fn get(&self, id: AssetId) -> Option<&AssetEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &AssetEntry> {
        self.entries.iter()
    }
}

impl fmt::Display for AssetManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AssetManifest({} entries)", self.entries.len())
    }
}

/// Range reads over the asset cache.
///
/// Replaces `env.wasm_httpfs_open`, `env.wasm_httpfs_size` and
/// `env.wasm_httpfs_pread`. Semantics the engine relies on:
///
/// * `pread` never writes past `buf.len()`, may return a short read, and
///   returns `Ok(0)` at or past EOF;
/// * `pread` at an offset greater than the recorded size is not an error —
///   probing past EOF is how the streamer detects truncation.
pub trait AssetStore: Send + Sync {
    fn manifest(&self) -> &AssetManifest;

    /// `env.wasm_httpfs_size` — recorded byte length, not a fresh `stat`.
    fn size(&self, id: AssetId) -> HostResult<u64>;

    /// `env.wasm_httpfs_pread` — read up to `buf.len()` bytes at `offset`.
    fn pread(&self, id: AssetId, offset: u64, buf: &mut [u8]) -> HostResult<usize>;

    /// Convenience for whole-asset reads (RPF index, meta files). Errors with
    /// [`HostError::OutOfRange`] instead of silently truncating.
    fn read_all(&self, id: AssetId, limit: usize) -> HostResult<Vec<u8>> {
        let size = self.size(id)?;
        let size = usize::try_from(size)
            .map_err(|_| HostError::OutOfRange(format!("asset {id} is larger than usize")))?;
        if size > limit {
            return Err(HostError::OutOfRange(format!(
                "asset {id} is {size} bytes, limit {limit}"
            )));
        }
        let mut buf = vec![0u8; size];
        let mut filled = 0;
        while filled < size {
            let n = self.pread(id, filled as u64, &mut buf[filled..])?;
            if n == 0 {
                return Err(HostError::OutOfRange(format!(
                    "asset {id} truncated at {filled}/{size}"
                )));
            }
            filled += n;
        }
        Ok(buf)
    }
}

/// Positional read on a `File` without a crate.
fn read_at_full(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    let mut done = 0;
    while done < buf.len() {
        let n = platform_read_at(file, &mut buf[done..], offset + done as u64)?;
        if n == 0 {
            break;
        }
        done += n;
    }
    Ok(done)
}

#[cfg(unix)]
fn platform_read_at(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::os::unix::fs::FileExt;
    file.read_at(buf, offset)
}

#[cfg(windows)]
fn platform_read_at(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::os::windows::fs::FileExt;
    file.seek_read(buf, offset)
}

#[cfg(not(any(unix, windows)))]
fn platform_read_at(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    // No positioned-read syscall: clone the handle and seek. Correct, only
    // slower, and never taken on supported targets.
    use std::io::{Read, Seek, SeekFrom};
    let mut handle = file.try_clone()?;
    handle.seek(SeekFrom::Start(offset))?;
    handle.read(buf)
}

/// Assets served straight from an extracted directory.
///
/// Replaces the `io_worker.js` `pread` path: `open` resolves a path through the
/// manifest, `size` answers from the manifest, `pread` hits the file. Handles
/// are opened once and cached — the engine keeps tens of thousands of asset ids
/// live during streaming.
pub struct DirAssetStore {
    root: PathBuf,
    manifest: AssetManifest,
    open: Mutex<HashMap<AssetId, Arc<File>>>,
}

impl DirAssetStore {
    pub fn new(root: impl Into<PathBuf>, manifest: AssetManifest) -> HostResult<Self> {
        let root = root.into();
        if !root.is_dir() {
            return Err(HostError::NotFound(format!(
                "asset root {} is not a directory",
                root.display()
            )));
        }
        Ok(Self {
            root,
            manifest,
            open: Mutex::new(HashMap::new()),
        })
    }

    /// Load `<root>/manifest.tsv` when present, otherwise start empty.
    pub fn from_root(root: impl Into<PathBuf>) -> HostResult<Self> {
        let root = root.into();
        let manifest_path = root.join("manifest.tsv");
        let manifest = match std::fs::read_to_string(&manifest_path) {
            Ok(text) => AssetManifest::parse(&text)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => match std::fs::read_to_string(root.join("manifest.json")) {
                Ok(text) => AssetManifest::parse_engine_json(&text)?,
                Err(e) if e.kind() == io::ErrorKind::NotFound => AssetManifest::default(),
                Err(e) => return Err(HostError::Io(e)),
            },
            Err(e) => return Err(HostError::Io(e)),
        };
        Self::new(root, manifest)
    }

    /// `env.wasm_httpfs_open` — map a path to a handle id.
    pub fn open(&self, path: &str) -> HostResult<AssetId> {
        self.manifest
            .lookup(path)
            .ok_or_else(|| HostError::NotFound(format!("asset {path}")))
    }

    fn entry(&self, id: AssetId) -> HostResult<&AssetEntry> {
        self.manifest
            .get(id)
            .ok_or_else(|| HostError::NotFound(format!("asset id {id}")))
    }

    fn handle(&self, id: AssetId) -> HostResult<Arc<File>> {
        let mut open = self
            .open
            .lock()
            .map_err(|_| HostError::Thread("asset handle cache poisoned".into()))?;
        if let Some(file) = open.get(&id) {
            return Ok(Arc::clone(file));
        }
        let entry = self.manifest.get(id).ok_or_else(|| {
            HostError::NotFound(format!("asset id {id}"))
        })?;
        let file = File::open(self.root.join(&entry.path)).map_err(|e| {
            HostError::Io(io::Error::new(
                e.kind(),
                format!("open asset {} ({id}): {e}", entry.path),
            ))
        })?;
        let file = Arc::new(file);
        open.insert(id, Arc::clone(&file));
        Ok(file)
    }
}

impl AssetStore for DirAssetStore {
    fn manifest(&self) -> &AssetManifest {
        &self.manifest
    }

    fn size(&self, id: AssetId) -> HostResult<u64> {
        self.entry(id).map(|e| e.size)
    }

    fn pread(&self, id: AssetId, offset: u64, buf: &mut [u8]) -> HostResult<usize> {
        let size = self.size(id)?;
        if buf.is_empty() || offset >= size {
            return Ok(0);
        }
        let want = buf.len().min((size - offset) as usize);
        let file = self.handle(id)?;
        let read = read_at_full(&file, &mut buf[..want], offset)?;
        Ok(read)
    }
}

impl fmt::Debug for DirAssetStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirAssetStore")
            .field("root", &self.root)
            .field("manifest", &self.manifest)
            .finish_non_exhaustive()
    }
}

/// Asset store that has nothing: every read is
/// [`HostError::Unsupported`], which tells the engine to stream over
/// [`crate::host::Host::http`] instead.
#[derive(Debug, Default)]
pub struct NullAssets {
    manifest: AssetManifest,
}

impl NullAssets {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AssetStore for NullAssets {
    fn manifest(&self) -> &AssetManifest {
        &self.manifest
    }

    fn size(&self, id: AssetId) -> HostResult<u64> {
        Err(HostError::NotFound(format!("asset id {id}")))
    }

    fn pread(&self, _id: AssetId, _offset: u64, _buf: &mut [u8]) -> HostResult<usize> {
        Err(HostError::Unsupported(
            "no local asset cache; stream through Host::http",
        ))
    }
}

/// Persistent per-title key/value blobs (saves, settings, profiles).
pub mod userdata {
    use super::*;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    const SUFFIX: &str = ".udata";
    const MAX_KEY: usize = 512;
    const MAX_PAGE: usize = 4096;

    /// One row of a page listing — `env.wasm_userdata_page_js` returns
    /// `(key, len)` pairs so the engine can show save slots without reading
    /// the bodies.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UserdataEntry {
        pub key: String,
        pub len: u32,
    }

    /// A page plus the cursor for the next one (`None` = end of listing).
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct UserdataPage {
        pub entries: Vec<UserdataEntry>,
        pub next: Option<String>,
    }

    /// Replaces `env.wasm_userdata_put_js`, `_get_js`, `_delete_js` and
    /// `_page_js` (IndexedDB in the browser build). Writes must be durable
    /// before `put` returns: the engine acknowledges saves to the UI
    /// immediately afterwards.
    pub trait UserdataStore: Send + Sync {
        fn put(&self, key: &str, bytes: &[u8]) -> HostResult<()>;
        fn get(&self, key: &str) -> HostResult<Option<Vec<u8>>>;
        /// Returns whether a blob existed.
        fn delete(&self, key: &str) -> HostResult<bool>;
        /// Keys strictly greater than `cursor`, sorted, at most `limit`.
        fn page(&self, cursor: Option<&str>, limit: usize) -> HostResult<UserdataPage>;
    }

    fn validate_key(key: &str) -> HostResult<()> {
        if key.is_empty() {
            return Err(HostError::InvalidArgument("empty userdata key".into()));
        }
        if key.len() > MAX_KEY {
            return Err(HostError::InvalidArgument(format!(
                "userdata key is {} bytes, max {MAX_KEY}",
                key.len()
            )));
        }
        if key.contains('\0') {
            return Err(HostError::InvalidArgument(
                "userdata key contains NUL".into(),
            ));
        }
        Ok(())
    }

    /// Percent-encode everything outside `[A-Za-z0-9._-]` so an arbitrary
    /// engine key is a legal, collision-free filename on every platform.
    fn encode_key(key: &str) -> String {
        let mut out = String::with_capacity(key.len());
        for byte in key.as_bytes() {
            let b = *byte;
            let plain = b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_');
            if plain {
                out.push(b as char);
            } else {
                out.push_str(&format!("%{b:02X}"));
            }
        }
        out
    }

    fn decode_key(name: &str) -> Option<String> {
        let mut bytes = Vec::with_capacity(name.len());
        let raw = name.as_bytes();
        let mut i = 0;
        while i < raw.len() {
            if raw[i] == b'%' {
                let hex = name.get(i + 1..i + 3)?;
                bytes.push(u8::from_str_radix(hex, 16).ok()?);
                i += 3;
            } else {
                bytes.push(raw[i]);
                i += 1;
            }
        }
        String::from_utf8(bytes).ok()
    }

    /// Directory-backed userdata: one file per key, written via
    /// temp-file + rename so a crash mid-save cannot leave a torn blob.
    pub struct DirUserdata {
        root: PathBuf,
        seq: AtomicU64,
    }

    impl DirUserdata {
        pub fn new(root: impl Into<PathBuf>) -> Self {
            Self {
                root: root.into(),
                seq: AtomicU64::new(0),
            }
        }

        pub fn root(&self) -> &Path {
            &self.root
        }

        fn path_for(&self, key: &str) -> PathBuf {
            self.root.join(format!("{}{SUFFIX}", encode_key(key)))
        }

        fn ensure_root(&self) -> HostResult<()> {
            std::fs::create_dir_all(&self.root)?;
            Ok(())
        }
    }

    impl UserdataStore for DirUserdata {
        fn put(&self, key: &str, bytes: &[u8]) -> HostResult<()> {
            validate_key(key)?;
            self.ensure_root()?;
            let target = self.path_for(key);
            let nonce = self.seq.fetch_add(1, Ordering::Relaxed);
            let tmp = self.root.join(format!(
                ".tmp-{}-{nonce}",
                std::process::id()
            ));
            {
                let mut file = File::create(&tmp)?;
                file.write_all(bytes)?;
                file.sync_all()?;
            }
            std::fs::rename(&tmp, &target).inspect_err(|_| {
                let _ = std::fs::remove_file(&tmp);
            })?;
            Ok(())
        }

        fn get(&self, key: &str) -> HostResult<Option<Vec<u8>>> {
            validate_key(key)?;
            match std::fs::read(self.path_for(key)) {
                Ok(bytes) => Ok(Some(bytes)),
                Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(HostError::Io(e)),
            }
        }

        fn delete(&self, key: &str) -> HostResult<bool> {
            validate_key(key)?;
            match std::fs::remove_file(self.path_for(key)) {
                Ok(()) => Ok(true),
                Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
                Err(e) => Err(HostError::Io(e)),
            }
        }

        fn page(&self, cursor: Option<&str>, limit: usize) -> HostResult<UserdataPage> {
            let limit = limit.clamp(1, MAX_PAGE);
            let dir = match std::fs::read_dir(&self.root) {
                Ok(dir) => dir,
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    return Ok(UserdataPage::default())
                }
                Err(e) => return Err(HostError::Io(e)),
            };
            let mut keys: Vec<(String, u32)> = Vec::new();
            for entry in dir {
                let entry = entry?;
                let name = entry.file_name();
                let Some(name) = name.to_str() else { continue };
                let Some(stem) = name.strip_suffix(SUFFIX) else {
                    continue;
                };
                let Some(key) = decode_key(stem) else { continue };
                let len = entry
                    .metadata()
                    .map(|m| u32::try_from(m.len()).unwrap_or(u32::MAX))
                    .unwrap_or(0);
                keys.push((key, len));
            }
            keys.sort_by(|a, b| a.0.cmp(&b.0));
            let start = match cursor {
                None => 0,
                Some(c) => keys.partition_point(|(k, _)| k.as_str() <= c),
            };
            let end = (start + limit).min(keys.len());
            let entries: Vec<UserdataEntry> = keys[start..end]
                .iter()
                .map(|(key, len)| UserdataEntry {
                    key: key.clone(),
                    len: *len,
                })
                .collect();
            let next = if end < keys.len() {
                entries.last().map(|e| e.key.clone())
            } else {
                None
            };
            Ok(UserdataPage { entries, next })
        }
    }

    impl fmt::Debug for DirUserdata {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DirUserdata")
                .field("root", &self.root)
                .finish_non_exhaustive()
        }
    }

    /// Userdata that forgets everything — used by the browser stub and by
    /// headless runs that must not touch disk.
    #[derive(Debug, Default)]
    pub struct NullUserdata;

    impl UserdataStore for NullUserdata {
        fn put(&self, key: &str, _bytes: &[u8]) -> HostResult<()> {
            validate_key(key)
        }

        fn get(&self, _key: &str) -> HostResult<Option<Vec<u8>>> {
            Ok(None)
        }

        fn delete(&self, _key: &str) -> HostResult<bool> {
            Ok(false)
        }

        fn page(&self, _cursor: Option<&str>, _limit: usize) -> HostResult<UserdataPage> {
            Ok(UserdataPage::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::userdata::{DirUserdata, UserdataStore};
    use super::*;

    const MANIFEST: &str = "\
# id\tsize\tpath
0\t12\tcommon/data/tiny.bin
1\t5\ttiny/two.bin
";

    fn store(dir: &crate::testing::TempDir) -> DirAssetStore {
        std::fs::write(dir.join("a.bin"), b"hello world!").expect("write a");
        std::fs::write(dir.join("b.bin"), b"abcde").expect("write b");
        let manifest = AssetManifest::parse(MANIFEST).expect("manifest");
        // Point the manifest entries at the files we just wrote.
        let manifest = AssetManifest::parse(
            &MANIFEST
                .replace("common/data/tiny.bin", "a.bin")
                .replace("tiny/two.bin", "b.bin"),
        )
        .expect("rewritten manifest");
        let _ = manifest;
        DirAssetStore::new(dir.path(), manifest).expect("store")
    }

    #[test]
    fn manifest_round_trips_and_looks_up_paths() {
        let manifest = AssetManifest::parse(MANIFEST).expect("parse");
        assert_eq!(manifest.len(), 2);
        assert_eq!(manifest.lookup("common/data/tiny.bin"), Some(0));
        assert_eq!(manifest.lookup("nope"), None);
        assert_eq!(manifest.get(1).map(|e| e.size), Some(5));
        assert_eq!(manifest.get(9), None);
        let reparsed = AssetManifest::parse(&manifest.to_text()).expect("reparse");
        assert_eq!(reparsed, manifest);
    }

    #[test]
    fn engine_json_round_trips_paths_and_index_based_ids() {
        let source = r#"{"version":"test","files":[["a\"b.bin",12],["x/y.bin",4294967297]]}"#;
        let manifest = AssetManifest::parse_engine_json(source).unwrap();
        assert_eq!(manifest.lookup("a\"b.bin"), Some(0));
        assert_eq!(manifest.get(1).unwrap().size, 4294967297);
        assert_eq!(AssetManifest::parse_engine_json(&manifest.to_engine_json().unwrap()).unwrap(), manifest);
        assert!(AssetManifest::parse_engine_json(r#"{"files":[["a",1],["a",2]]}"#).is_err());
        assert!(AssetManifest::parse_engine_json(r#"{"files":[["a",-1]]}"#).is_err());
        assert!(AssetManifest::parse("2\t12\ta.bin\n").unwrap().to_engine_json().is_err());
    }

    #[test]
    fn manifest_rejects_malformed_lines() {
        assert!(AssetManifest::parse("0\t1\n").is_err());
        assert!(AssetManifest::parse("x\t1\tpath\n").is_err());
        assert!(AssetManifest::parse("0\tz\tpath\n").is_err());
        let dup = "0\t1\tp\n1\t1\tp\n";
        assert!(matches!(
            AssetManifest::parse(dup),
            Err(HostError::Manifest(_))
        ));
    }

    #[test]
    fn pread_returns_the_requested_window() {
        let dir = crate::testing::TempDir::new("assets").expect("tempdir");
        let store = store(&dir);
        assert_eq!(store.size(0).expect("size"), 12);

        let mut buf = [0u8; 12];
        assert_eq!(store.pread(0, 0, &mut buf).expect("head"), 12);
        assert_eq!(&buf, b"hello world!");

        let mut mid = [0u8; 5];
        assert_eq!(store.pread(0, 4, &mut mid).expect("mid"), 5);
        assert_eq!(&mid, b"o wor");

        // Short read clamps to EOF rather than erroring.
        let mut tail = [0u8; 8];
        assert_eq!(store.pread(0, 8, &mut tail).expect("tail"), 4);
        assert_eq!(&tail[..4], b"rld!");

        // At and past EOF both answer 0.
        let mut eof = [0u8; 4];
        assert_eq!(store.pread(0, 12, &mut eof).expect("eof"), 0);
        assert_eq!(store.pread(0, 99, &mut eof).expect("past eof"), 0);
        assert_eq!(store.read_all(0, 64).expect("read_all"), b"hello world!");
        assert!(matches!(
            store.read_all(0, 4),
            Err(HostError::OutOfRange(_))
        ));
    }

    #[test]
    fn pread_rejects_unknown_ids_and_missing_files() {
        let dir = crate::testing::TempDir::new("assets-err").expect("tempdir");
        let store = store(&dir);
        let mut buf = [0u8; 4];
        assert!(matches!(
            store.pread(7, 0, &mut buf),
            Err(HostError::NotFound(_))
        ));
        std::fs::remove_file(dir.join("b.bin")).expect("remove");
        let other = DirAssetStore::new(dir.path(), store.manifest().clone()).expect("store2");
        assert!(matches!(other.pread(1, 0, &mut buf), Err(HostError::Io(_))));
        assert!(DirAssetStore::new(dir.join("missing"), AssetManifest::default()).is_err());
    }

    #[test]
    fn userdata_persists_across_reopen() {
        let dir = crate::testing::TempDir::new("userdata").expect("tempdir");
        let root = dir.join("ud");
        {
            let store = DirUserdata::new(&root);
            store.put("save/slot 1", b"slot-one").expect("put");
            store.put("settings", b"{\"fov\":95}").expect("put");
            assert_eq!(
                store.get("save/slot 1").expect("get").as_deref(),
                Some(&b"slot-one"[..])
            );
            // Overwrite is atomic and visible immediately.
            store.put("save/slot 1", b"slot-one-v2").expect("put2");
            assert_eq!(
                store.get("save/slot 1").expect("get").as_deref(),
                Some(&b"slot-one-v2"[..])
            );
        }
        // Fresh instance: the data came from disk, not memory.
        let reopened = DirUserdata::new(&root);
        assert_eq!(
            reopened.get("save/slot 1").expect("get").as_deref(),
            Some(&b"slot-one-v2"[..])
        );
        assert_eq!(reopened.get("absent").expect("get"), None);
        assert!(reopened.delete("save/slot 1").expect("delete"));
        assert!(!reopened.delete("save/slot 1").expect("delete twice"));
        assert_eq!(reopened.get("save/slot 1").expect("get"), None);
        assert_eq!(
            reopened.get("settings").expect("get").as_deref(),
            Some(&b"{\"fov\":95}"[..])
        );
    }

    #[test]
    fn userdata_pages_are_sorted_and_cursored() {
        let dir = crate::testing::TempDir::new("userdata-page").expect("tempdir");
        let store = DirUserdata::new(dir.join("ud"));
        for key in ["a", "b", "c", "d", "e"] {
            store.put(key, key.as_bytes()).expect("put");
        }
        let first = store.page(None, 2).expect("page1");
        assert_eq!(
            first.entries.iter().map(|e| e.key.as_str()).collect::<Vec<_>>(),
            ["a", "b"]
        );
        assert_eq!(first.next.as_deref(), Some("b"));
        let second = store.page(first.next.as_deref(), 2).expect("page2");
        assert_eq!(
            second
                .entries
                .iter()
                .map(|e| e.key.as_str())
                .collect::<Vec<_>>(),
            ["c", "d"]
        );
        let last = store.page(second.next.as_deref(), 2).expect("page3");
        assert_eq!(last.entries.len(), 1);
        assert_eq!(last.next, None);
        assert_eq!(last.entries[0].len, 1);

        // An absent root pages as empty instead of failing.
        let empty = DirUserdata::new(dir.join("never-created"));
        assert!(empty.page(None, 4).expect("empty").entries.is_empty());
        // Hostile keys are rejected, not sanitised into collisions.
        assert!(store.put("", b"x").is_err());
        assert!(store.get("bad\0key").is_err());
    }

    #[test]
    fn userdata_keys_are_escaped_without_collisions() {
        let dir = crate::testing::TempDir::new("userdata-keys").expect("tempdir");
        let store = DirUserdata::new(dir.join("ud"));
        store.put("a/b", b"slash").expect("put");
        store.put("a%2Fb", b"percent").expect("put");
        assert_eq!(store.get("a/b").expect("get").as_deref(), Some(&b"slash"[..]));
        assert_eq!(
            store.get("a%2Fb").expect("get").as_deref(),
            Some(&b"percent"[..])
        );
        assert_eq!(store.page(None, 10).expect("page").entries.len(), 2);
    }
}
