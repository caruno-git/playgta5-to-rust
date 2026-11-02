use std::{env, error::Error, fs, path::PathBuf};
use sha2::{Digest, Sha256};
use wasm_encoder::{ExportKind, ExportSection, Module, RawSection};
use wasmparser::{ExternalKind, Parser, Payload, Validator, WasmFeatures};

fn sections_digest(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut hash = Sha256::new();
    for payload in Parser::new(0).parse_all(bytes) {
        let payload = payload?;
        if let Some((id, range)) = payload.as_section() {
            if id != 7 {
                hash.update([id]);
                hash.update((range.len() as u64).to_le_bytes());
                hash.update(&bytes[range]);
            }
        }
    }
    Ok(hash.finalize().to_vec())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let input = PathBuf::from(args.next().ok_or("expected input.wasm output.wasm")?);
    let output = PathBuf::from(args.next().ok_or("expected output.wasm")?);
    if args.next().is_some() { return Err("unexpected argument".into()); }
    if input.canonicalize()? == output.canonicalize().unwrap_or_else(|_| output.clone()) { return Err("input and output must differ".into()); }
    let bytes = fs::read(&input)?;
    Validator::new_with_features(WasmFeatures::all()).validate_all(&bytes)?;
    let mut module = Module::new();
    let mut before = 0;
    let mut kept = Vec::new();
    for payload in Parser::new(0).parse_all(&bytes) {
        let payload = payload?;
        if let Payload::ExportSection(reader) = payload {
            let mut exports = ExportSection::new();
            for export in reader {
                let export = export?;
                before += 1;
                if export.name.starts_with("engine_") || matches!(export.name, "__stack_pointer" | "__indirect_function_table" | "memory") {
                    let kind = match export.kind {
                        ExternalKind::Func => ExportKind::Func,
                        ExternalKind::Table => ExportKind::Table,
                        ExternalKind::Memory => ExportKind::Memory,
                        ExternalKind::Global => ExportKind::Global,
                        ExternalKind::Tag => ExportKind::Tag,
                    };
                    exports.export(export.name, kind, export.index);
                    kept.push(export.name.to_owned());
                }
            }
            module.section(&exports);
        } else if let Some((id, range)) = payload.as_section() {
            module.section(&RawSection { id, data: &bytes[range] });
        }
    }
    if !kept.iter().any(|n| n == "engine_new_context") || !kept.iter().any(|n| n == "engine___wasm_call_ctors") { return Err("not a Rust engine module".into()); }
    let packed = module.finish();
    Validator::new_with_features(WasmFeatures::all()).validate_all(&packed)?;
    if sections_digest(&bytes)? != sections_digest(&packed)? { return Err("non-export sections changed".into()); }
    fs::write(&output, &packed)?;
    let report = serde_json::json!({
        "author":"carunodev @stfu", "exports_before":before, "exports_after":kept.len(),
        "input_bytes":bytes.len(), "output_bytes":packed.len(), "non_export_sections_identical":true,
        "sha256":format!("{:x}", Sha256::digest(&packed)), "exports":kept,
    });
    fs::write(output.with_extension("exports.json"), serde_json::to_vec_pretty(&report)?)?;
    println!("PASS: exports {before} -> {}, bytes {} -> {}; all non-export sections identical", report["exports_after"], bytes.len(), packed.len());
    Ok(())
}

fn main() { if let Err(error) = run() { eprintln!("wasm package: {error}"); std::process::exit(1); } }
