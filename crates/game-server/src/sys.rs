//! Small OS shims: resident set size and page size.
//!
//! Windows uses `GetProcessMemoryInfo` (kernel32, no crate needed); Linux reads
//! `/proc/self/statm`. Other platforms report 0 so the status line stays
//! well-formed rather than failing.

/// Resident set size in kibibytes, or 0 when the platform does not expose it.
pub fn rss_kb() -> u64 {
    platform::rss_kb()
}

/// Native page size in bytes.
pub fn page_size() -> u64 {
    platform::page_size()
}

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Default)]
    struct ProcessMemoryCounters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set_size: usize,
        working_set_size: usize,
        quota_peak_paged_pool_usage: usize,
        quota_paged_pool_usage: usize,
        quota_peak_non_paged_pool_usage: usize,
        quota_non_paged_pool_usage: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetCurrentProcess() -> *mut c_void;
        fn K32GetProcessMemoryInfo(
            process: *mut c_void,
            counters: *mut ProcessMemoryCounters,
            cb: u32,
        ) -> i32;
        fn GetSystemInfo(info: *mut SystemInfo);
    }

    #[repr(C)]
    #[derive(Default)]
    struct SystemInfo {
        processor_architecture: u16,
        reserved: u16,
        page_size: u32,
        minimum_application_address: *mut c_void,
        maximum_application_address: *mut c_void,
        active_processor_mask: usize,
        number_of_processors: u32,
        processor_type: u32,
        allocation_granularity: u32,
        processor_level: u16,
        processor_revision: u16,
    }

    pub fn rss_kb() -> u64 {
        let mut counters = ProcessMemoryCounters {
            cb: std::mem::size_of::<ProcessMemoryCounters>() as u32,
            ..Default::default()
        };
        // SAFETY: the counter struct is fully initialised (cb set) and lives on
        // this stack frame for the duration of the call.
        let ok = unsafe {
            K32GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut counters,
                std::mem::size_of::<ProcessMemoryCounters>() as u32,
            )
        };
        if ok == 0 {
            0
        } else {
            (counters.working_set_size / 1024) as u64
        }
    }

    pub fn page_size() -> u64 {
        let mut info = SystemInfo::default();
        // SAFETY: GetSystemInfo fills a caller-provided SYSTEM_INFO.
        unsafe { GetSystemInfo(&mut info) };
        if info.page_size == 0 {
            4096
        } else {
            info.page_size as u64
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod platform {
    pub fn rss_kb() -> u64 {
        let Ok(statm) = std::fs::read_to_string("/proc/self/statm") else {
            return 0;
        };
        let mut fields = statm.split_whitespace();
        let _size = fields.next();
        let Some(resident) = fields.next() else {
            return 0;
        };
        let Ok(pages) = resident.parse::<u64>() else {
            return 0;
        };
        pages.saturating_mul(4) // 4 KiB pages on every Linux we target
    }

    pub fn page_size() -> u64 {
        // SAFETY: sysconf(_SC_PAGESIZE) takes no memory and cannot fault.
        let size = unsafe { sysconf(30) }; // _SC_PAGESIZE
        if size > 0 {
            size as u64
        } else {
            4096
        }
    }

    extern "C" {
        fn sysconf(name: i32) -> i64;
    }
}

#[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
mod platform {
    pub fn rss_kb() -> u64 {
        0
    }

    pub fn page_size() -> u64 {
        4096
    }
}