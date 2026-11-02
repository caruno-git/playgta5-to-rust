//! Determinism harness for the fixed-tick server runtime.
//!
//! This integration-test support module is deliberately self-contained: the
//! checkout does not contain a `wasm-rt` crate, so [`vm`] models the linear
//! memory the port plan describes (a 3 GiB / 49_152-page sparse reservation,
//! 4_077 passive segments) rather than linking a real wasm runtime. Everything
//! that can be taken from `engine_runtime` is: the virtual [`Clock`], the fixed
//! step from [`TickConfig`], and [`StateHasher`].
//!
//! See `recovered/rust/build/DETERMINISM.md` for the methodology, the excluded
//! regions and the measured result table.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod futex;
pub mod runner;
pub mod simd;
pub mod vm;
pub mod workloads;

pub use runner::{
    run_spec, run_workload, Digest, FnWorkload, RunConfig, RunRecord, Spec, TickInfo, WasmFn,
    Workload, WorkloadKind,
};
pub use vm::{
    excluded, process_nonce, StateDigest, Vm, DATA_BASE, FUTEX_BASE, PAGE_SIZE, RESERVED_BYTES,
    RESERVED_PAGES, SCRATCH_BASE, SEGMENT_COUNT, STACK_BASE, STACK_END, TLS_BASE, TLS_END,
};
