//! Asset presence probe.
//!
//! The engine needs the retail archives (`common.rpf`, `x64a..x64w.rpf`, the
//! `update/` and `platform/` trees) before any real gameplay tick can do work.
//! None of that ships with this repository, so the probe classifies the data
//! dir and the server degrades to a logged, non-fatal "assets absent" state
//! instead of refusing to boot.

use std::fmt;
use std::path::{Path, PathBuf};

/// Files that the retail layout must contain for a full boot.
const REQUIRED: [&str; 3] = ["common.rpf", "update/update.rpf", "x64a.rpf"];

#[derive(Debug, Clone)]
pub struct AssetReport {
    pub dir: PathBuf,
    pub present: bool,
    pub rpf_files: usize,
    pub total_bytes: u64,
    pub missing: Vec<&'static str>,
}

impl AssetReport {
    pub fn is_present(&self) -> bool {
        self.present
    }

    /// Short token used in the status line: `present` | `partial` | `missing`.
    pub fn state(&self) -> &'static str {
        if self.present {
            "present"
        } else if self.rpf_files > 0 {
            "partial"
        } else {
            "missing"
        }
    }

    /// One human-readable line for the boot log.
    pub fn summary(&self) -> String {
        if self.present {
            format!(
                "assets present in {} ({} archives, {:.1} MiB)",
                self.dir.display(),
                self.rpf_files,
                self.total_bytes as f64 / (1024.0 * 1024.0)
            )
        } else {
            format!(
                "assets absent in {} ({} archives, missing: {})",
                self.dir.display(),
                self.rpf_files,
                self.missing.join(", ")
            )
        }
    }
}

impl fmt::Display for AssetReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.summary())
    }
}

pub fn probe(dir: &Path) -> AssetReport {
    let mut missing = Vec::new();
    for rel in REQUIRED {
        if !dir.join(rel).is_file() {
            missing.push(rel);
        }
    }

    // Count archives one level deep; that covers both the flat retail layout
    // and the `update/x64/data/` style nesting used by modded installs.
    let mut rpf_files = 0usize;
    let mut total_bytes = 0u64;
    count_archives(dir, 0, &mut rpf_files, &mut total_bytes);

    AssetReport {
        dir: dir.to_path_buf(),
        present: missing.is_empty(),
        rpf_files,
        total_bytes,
        missing,
    }
}

fn count_archives(dir: &Path, depth: usize, files: &mut usize, bytes: &mut u64) {
    if depth > 2 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(meta) = entry.metadata() else { continue };
        let path = entry.path();
        if meta.is_dir() {
            count_archives(&path, depth + 1, files, bytes);
        } else if meta.is_file() {
            let is_archive = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("rpf"))
                .unwrap_or(false);
            if is_archive {
                *files += 1;
                *bytes += meta.len();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("gs-assets-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn empty_dir_is_missing_not_an_error() {
        let dir = scratch("empty");
        let report = probe(&dir);
        assert!(!report.is_present());
        assert_eq!(report.state(), "missing");
        assert_eq!(report.rpf_files, 0);
        assert_eq!(report.missing.len(), REQUIRED.len());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn full_layout_is_present() {
        let dir = scratch("full");
        fs::create_dir_all(dir.join("update")).unwrap();
        for rel in REQUIRED {
            let path = dir.join(rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, b"rpf").unwrap();
        }
        let report = probe(&dir);
        assert!(report.is_present(), "{report:?}");
        assert_eq!(report.state(), "present");
        assert_eq!(report.rpf_files, 3);
        assert_eq!(report.total_bytes, 9);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn lone_archive_reports_partial() {
        let dir = scratch("partial");
        fs::write(dir.join("common.rpf"), b"xx").unwrap();
        let report = probe(&dir);
        assert!(!report.is_present());
        assert_eq!(report.state(), "partial");
        let _ = fs::remove_dir_all(&dir);
    }
}