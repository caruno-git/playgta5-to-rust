//! Determinism acceptance tests for the fixed-tick server runtime.
//!
//! Run with output:
//!
//! ```text
//! cargo test -p engine-runtime --test determinism -- --nocapture --test-threads=1
//! ```
//!
//! The crate is standalone (`[workspace]` in its manifest), so from
//! `crates/engine-runtime` use:
//!
//! ```text
//! cargo test --test determinism -- --nocapture --test-threads=1
//! ```
//!
//! The table printed by [`full_report`] is reproduced in
//! `recovered/rust/build/DETERMINISM.md`.

mod harness;

use std::collections::HashMap;
use std::env;
use std::process::Command;

use harness::{
    run_spec, Digest, RunConfig, RunRecord, Spec, Vm, WasmFn, DATA_BASE, PAGE_SIZE, STACK_BASE,
    TLS_BASE,
};

/// 300 ticks at the runtime's default 60 Hz step = 5 simulated seconds.
const CFG: RunConfig = RunConfig {
    ticks: 300,
    step_ns: 16_666_667,
};

/// Every workload, including the control.
const ALL: [WasmFn; 6] = [
    WasmFn::Arithmetic,
    WasmFn::Memory,
    WasmFn::Simd,
    WasmFn::Futex,
    WasmFn::Mixed,
    WasmFn::WallClock,
];

/// Set for the re-executed child process.
const CHILD_ENV: &str = "ENGINE_DETERMINISM_CHILD";

fn run_once(func: WasmFn) -> RunRecord {
    let mut spec = Spec::builtin(func);
    run_spec(&mut spec, CFG)
}

fn hex(v: u64) -> String {
    format!("{v:016x}")
}

fn all_equal(d: &[Digest; 4]) -> bool {
    d[0] == d[1] && d[1] == d[2] && d[2] == d[3]
}

fn every_run(d: &[Digest; 4], field: impl Fn(&Digest) -> u64) -> bool {
    d.iter().all(|x| field(x) == field(&d[0]))
}

#[test]
fn deterministic_workloads_repeat_in_process() {
    for func in ALL {
        if !func.expected_deterministic() {
            continue;
        }
        let a = run_once(func);
        let b = run_once(func);
        assert_eq!(
            a.digest,
            b.digest,
            "{} diverged between two in-process runs\nA={:?}\nB={:?}",
            func.name(),
            a.digest,
            b.digest
        );
        assert_eq!(a.frame, CFG.ticks as i32);
        assert_eq!(a.ticks, CFG.ticks);
    }
}

#[test]
fn wall_clock_control_diverges() {
    let runs: Vec<Digest> = (0..5).map(|_| run_once(WasmFn::WallClock).digest).collect();
    let distinct = runs.iter().filter(|d| **d != runs[0]).count();
    assert!(
        distinct > 0,
        "wall-clock control produced only one digest across 5 runs: {:?}",
        runs
    );
}

#[test]
fn mixed_workload_reaches_every_subsystem() {
    let record = run_once(WasmFn::Mixed);
    assert!(
        record.pages >= 8,
        "mixed workload allocated only {} pages",
        record.pages
    );
    assert!(
        record.futex_events > 0,
        "futex path logged no wake/timeout events"
    );
    assert_ne!(record.digest.memory, 0);
    assert_ne!(record.digest.registers, 0);
    assert_ne!(record.digest.globals, 0);
    assert_ne!(record.digest.futex, 0);
    assert_eq!(record.frame, CFG.ticks as i32);
}

#[test]
fn closure_workloads_are_supported() {
    fn counter() -> Spec {
        Spec::closure("closure_counter", |vm, info| {
            vm.regs[20] = vm.regs[20].wrapping_add(info.index);
            vm.write_u64(DATA_BASE + 0x9000, vm.regs[20]);
            vm.pc = 0xC105;
        })
    }
    let a = run_spec(&mut counter(), CFG);
    let b = run_spec(&mut counter(), CFG);
    assert_eq!(a.digest, b.digest);
    assert_eq!(a.fn_id, None);
    assert_eq!(a.name, "closure_counter");
}

#[test]
fn excluded_regions_do_not_affect_the_digest() {
    let a = Vm::boot(0x1111_1111_1111_1111);
    let b = Vm::boot(0x2222_2222_2222_2222);
    assert_eq!(a.memory_digest(), b.memory_digest());

    let stack_page = STACK_BASE / PAGE_SIZE as u32;
    let tls_page = TLS_BASE / PAGE_SIZE as u32;
    assert_ne!(
        a.raw_page(stack_page),
        b.raw_page(stack_page),
        "boot nonce was not written to the excluded stack page"
    );
    let hashed: Vec<u32> = a.page_hashes().into_iter().map(|(p, _)| p).collect();
    assert!(!hashed.contains(&stack_page), "stack page was hashed");
    assert!(!hashed.contains(&tls_page), "TLS page was hashed");
}

#[test]
fn page_granular_hashing_detects_one_byte() {
    let before = Vm::boot(7);
    let mut after = Vm::boot(7);
    after.write(DATA_BASE + 1234, &[0xA5]);
    assert_ne!(before.memory_digest(), after.memory_digest());

    let bh = before.page_hashes();
    let ah = after.page_hashes();
    assert_eq!(bh.len(), ah.len());
    let changed = bh.iter().zip(&ah).filter(|(x, y)| x != y).count();
    assert_eq!(changed, 1, "exactly one page hash should change");
}

/// Re-executed child: prints one tab-separated digest line per workload.
/// Does nothing in a normal test-suite run.
#[test]
fn child_export() {
    if env::var(CHILD_ENV).ok().as_deref() != Some("1") {
        return;
    }
    for func in ALL {
        let r = run_once(func);
        println!(
            "DET_RESULT\t{}\t{:016x}\t{:016x}\t{:016x}\t{:016x}\t{:016x}",
            func.name(),
            r.digest.memory,
            r.digest.registers,
            r.digest.globals,
            r.digest.futex,
            r.digest.combined
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct ChildDigests {
    memory: u64,
    registers: u64,
    globals: u64,
    futex: u64,
    combined: u64,
}

impl ChildDigests {
    fn digest(self) -> Digest {
        Digest {
            memory: self.memory,
            registers: self.registers,
            globals: self.globals,
            futex: self.futex,
            combined: self.combined,
        }
    }
}

fn parse_hex(s: &str) -> u64 {
    u64::from_str_radix(s, 16).unwrap_or_else(|e| panic!("bad hex {s:?}: {e}"))
}

fn spawn_child() -> HashMap<String, ChildDigests> {
    let exe = env::current_exe().expect("current_exe");
    let out = Command::new(&exe)
        .args(["--exact", "child_export", "--nocapture", "--test-threads=1"])
        .env(CHILD_ENV, "1")
        .output()
        .expect("spawn determinism child");
    assert!(
        out.status.success(),
        "child failed: {}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut map = HashMap::new();
    for line in stdout.lines() {
        let Some(pos) = line.find("DET_RESULT\t") else {
            continue;
        };
        let rest = &line[pos + "DET_RESULT\t".len()..];
        let f: Vec<&str> = rest.split('\t').collect();
        assert_eq!(f.len(), 6, "malformed child line: {line:?}");
        map.insert(
            f[0].to_string(),
            ChildDigests {
                memory: parse_hex(f[1]),
                registers: parse_hex(f[2]),
                globals: parse_hex(f[3]),
                futex: parse_hex(f[4]),
                combined: parse_hex(f[5]),
            },
        );
    }
    assert_eq!(
        map.len(),
        ALL.len(),
        "expected {} child records, got {}\nstdout:\n{stdout}",
        ALL.len(),
        map.len()
    );
    map
}

/// The full matrix: two runs in this process, two separate child processes.
#[test]
fn full_report() {
    assert_eq!(
        CFG.step_ns,
        engine_runtime::TickConfig::default().step_ns(),
        "harness step drifted from the runtime's default"
    );
    let mut rows: Vec<(WasmFn, [Digest; 4])> = Vec::new();

    for func in ALL {
        let a = run_once(func).digest;
        let b = run_once(func).digest;
        rows.push((func, [a, b, a, b]));
    }

    let child_a = spawn_child();
    let child_b = spawn_child();
    for (func, row) in &mut rows {
        row[2] = child_a
            .get(func.name())
            .unwrap_or_else(|| panic!("child A missing {}", func.name()))
            .digest();
        row[3] = child_b
            .get(func.name())
            .unwrap_or_else(|| panic!("child B missing {}", func.name()))
            .digest();
    }

    println!("\n=== engine-runtime determinism report ===");
    println!(
        "{} ticks at {} ns/step ({} Hz), memory = page-granular, stack+TLS excluded\n",
        CFG.ticks,
        CFG.step_ns,
        (1_000_000_000u64 + CFG.step_ns / 2) / CFG.step_ns
    );
    println!(
        "{:<19} {:<23} {:>16} {:>16} {:>16} {:>16}  {}",
        "workload", "subsystem", "in-proc A", "in-proc B", "proc A", "proc B", "verdict"
    );
    println!("{}", "-".repeat(19 + 1 + 23 + 1 + 16 * 4 + 3 * 4 + 2 + 28));

    for (func, d) in &rows {
        let equal = all_equal(d);
        let verdict = if func.expected_deterministic() {
            if equal {
                "DETERMINISTIC"
            } else {
                "DIVERGENT (FAIL)"
            }
        } else if equal {
            "control: UNEXPECTEDLY STABLE"
        } else {
            "control: divergent (expected)"
        };
        println!(
            "{:<19} {:<23} {:>16} {:>16} {:>16} {:>16}  {}",
            func.name(),
            func.subsystem(),
            hex(d[0].combined),
            hex(d[1].combined),
            hex(d[2].combined),
            hex(d[3].combined),
            verdict
        );
    }
    println!();

    println!("component digests (in-proc A):\n");
    println!(
        "{:<19} {:>16} {:>16} {:>16} {:>16}",
        "workload", "memory", "registers", "globals", "futex"
    );
    for (func, d) in &rows {
        println!(
            "{:<19} {:>16} {:>16} {:>16} {:>16}",
            func.name(),
            hex(d[0].memory),
            hex(d[0].registers),
            hex(d[0].globals),
            hex(d[0].futex)
        );
    }
    println!();

    for (func, d) in &rows {
        if func.expected_deterministic() {
            assert!(
                all_equal(d),
                "{} is not deterministic across the 4 runs: {:?}",
                func.name(),
                d
            );
            for (label, stable) in [
                ("memory", every_run(d, |x| x.memory)),
                ("registers", every_run(d, |x| x.registers)),
                ("globals", every_run(d, |x| x.globals)),
                ("futex", every_run(d, |x| x.futex)),
            ] {
                assert!(
                    stable,
                    "{}: {label} digest diverged across the 4 runs",
                    func.name()
                );
            }
        } else {
            let any_diverged = !every_run(d, |x| x.memory)
                || !every_run(d, |x| x.registers)
                || !every_run(d, |x| x.globals)
                || !every_run(d, |x| x.futex);
            assert!(
                any_diverged,
                "wall-clock control was identical across all 4 runs; harness is not sensitive"
            );
        }
    }
}
