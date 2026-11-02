use std::{collections::HashMap, env, error::Error, fmt::Write, fs, path::PathBuf};
use wasmparser::{ElementItems, ElementKind, ExternalKind, Operator, Parser, Payload, TypeRef, ValType};

fn constant(expr: wasmparser::ConstExpr<'_>) -> Result<u64, Box<dyn Error>> {
    let mut reader = expr.get_operators_reader();
    let result = match reader.read()? {
        Operator::I64Const { value } => value as u64,
        Operator::I32Const { value } => value as u32 as u64,
        Operator::F32Const { value } => value.bits() as u64,
        Operator::F64Const { value } => value.bits(),
        op => return Err(format!("unsupported initializer {op:?}").into()),
    };
    if !matches!(reader.read()?, Operator::End) || !reader.eof() {
        return Err("initializer is not a single constant".into());
    }
    Ok(result)
}

fn value_type(ty: ValType) -> String {
    match ty {
        ValType::I32 => "i32", ValType::I64 => "i64", ValType::F32 => "f32",
        ValType::F64 => "f64", ValType::V128 => "v128", _ => panic!("reference type"),
    }.into()
}

fn run() -> Result<(), Box<dyn Error>> {
    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm64") {
        println!("cargo:rustc-link-arg=--max-memory=17179869184");
    }
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let wasm_path = env::var_os("PLAYGTA5_WASM").map(PathBuf::from).unwrap_or_else(|| root.join("../inputs/game.wasm"));
    let index_path = root.join("../build/index.json");
    println!("cargo:rerun-if-changed={}", wasm_path.display());
    println!("cargo:rerun-if-changed={}", index_path.display());
    let bytes = fs::read(wasm_path)?;
    let meta: serde_json::Value = serde_json::from_slice(&fs::read(index_path)?)?;
    let mut type_signatures = Vec::new();
    let mut canonical = HashMap::new();
    let mut function_types = Vec::new();
    let mut globals = Vec::new();
    let mut elements = Vec::new();
    let mut exports = Vec::new();
    let mut table_len = 0;
    let mut initial_pages = 0;
    let mut max_pages = 0;
    for payload in Parser::new(0).parse_all(&bytes) {
        match payload? {
            Payload::TypeSection(reader) => {
                for ty in reader.into_iter_err_on_gc_types() {
                    let ty = ty?;
                    let signature = (ty.params().iter().copied().map(value_type).collect::<Vec<_>>(),
                                     ty.results().iter().copied().map(value_type).collect::<Vec<_>>());
                    let id = type_signatures.len() as u32;
                    let original = &meta["types"][id.to_string()];
                    if serde_json::to_value(&signature)? != *original { return Err("type metadata differs from binary".into()); }
                    let id = *canonical.entry(signature.clone()).or_insert(id);
                    type_signatures.push(id);
                }
            }
            Payload::ImportSection(reader) => for import in reader {
                match import?.ty {
                    TypeRef::Func(ty) => function_types.push(ty),
                    TypeRef::Memory(memory) => {
                        if !memory.memory64 || !memory.shared { return Err("expected shared memory64".into()); }
                        initial_pages = memory.initial;
                        max_pages = memory.maximum.ok_or("missing memory maximum")?;
                    }
                    _ => return Err("unexpected import kind".into()),
                }
            },
            Payload::FunctionSection(reader) => for ty in reader { function_types.push(ty?); },
            Payload::ExportSection(reader) => for export in reader {
                let export = export?;
                if export.kind == ExternalKind::Func { exports.push((export.name.to_owned(), export.index)); }
            },
            Payload::GlobalSection(reader) => for global in reader { globals.push(constant(global?.init_expr)?); },
            Payload::TableSection(reader) => for table in reader {
                let table = table?;
                if table_len != 0 || !table.ty.table64 { return Err("expected one table64".into()); }
                table_len = table.ty.initial as usize;
            },
            Payload::ElementSection(reader) => for element in reader {
                let element = element?;
                let start = match element.kind {
                    ElementKind::Active { table_index, offset_expr } if table_index.unwrap_or(0) == 0 => constant(offset_expr)? as usize,
                    _ => return Err("unexpected element segment".into()),
                };
                match element.items {
                    ElementItems::Functions(reader) => for (i, function) in reader.into_iter().enumerate() { elements.push((start + i, function?)); },
                    _ => return Err("unexpected element expression".into()),
                }
            },
            _ => {}
        }
    }
    if globals.len() > 64 || function_types.len() != 91_111 || table_len != 89_368 { return Err("unexpected module dimensions".into()); }
    let names: HashMap<u32, &str> = meta["funcs"].as_array().ok_or("missing functions")?.iter()
        .map(|f| (f[0].as_u64().unwrap() as u32, f[7].as_str().unwrap())).collect();
    let mut table_types = vec![u32::MAX; table_len];
    let mut source = String::new();
    writeln!(source, "pub const INITIAL_PAGES: u64 = {initial_pages};")?;
    writeln!(source, "pub const MAX_PAGES: u64 = {max_pages};")?;
    writeln!(source, "pub const GLOBAL_INIT: &[u64] = &{globals:?};")?;
    writeln!(source, "pub fn make_table() -> Vec<*const ()> {{ let mut out = vec![core::ptr::null(); {table_len}];")?;
    for chunk in 0..elements.len().div_ceil(2048) { writeln!(source, "table_chunk_{chunk}(&mut out);")?; }
    writeln!(source, "out }}")?;
    for (chunk, entries) in elements.chunks(2048).enumerate() {
        writeln!(source, "fn table_chunk_{chunk}(out: &mut [*const ()]) {{")?;
        for &(slot, function) in entries {
            table_types[slot] = type_signatures[function_types[function as usize] as usize];
            if function < 85 {
                writeln!(source, "out[{slot}] = wasm_imp::table_import_{function} as *const ();")?;
            } else {
                let name = names.get(&function).ok_or("missing function name")?;
                let krate = (function - 85) / 4096;
                writeln!(source, "out[{slot}] = gen{krate:03}::{name}::{name} as *const ();")?;
            }
        }
        writeln!(source, "}}")?;
    }
    writeln!(source, "pub static TABLE_TYPES: [u32; {table_len}] = {table_types:?};")?;
    let out = PathBuf::from(env::var("OUT_DIR")?);
    fs::write(out.join("module.rs"), source)?;
    let rust_type = |ty: &str| if ty == "v128" { "u128".to_owned() } else { ty.to_owned() };
    let mut adapters = String::new();
    for (export, function) in &exports {
        let signature = &meta["types"][function_types[*function as usize].to_string()];
        let params: Vec<String> = signature[0].as_array().unwrap().iter().map(|t| rust_type(t.as_str().unwrap())).collect();
        let results: Vec<String> = signature[1].as_array().unwrap().iter().map(|t| rust_type(t.as_str().unwrap())).collect();
        let result = if results.is_empty() { String::new() } else if results.len() == 1 { format!(" -> {}", results[0]) } else { format!(" -> ({})", results.join(", ")) };
        let declarations = params.iter().enumerate().map(|(i, t)| format!("a{i}: {t}")).collect::<Vec<_>>().join(", ");
        let args = params.iter().enumerate().map(|(i, _)| format!(", a{i}")).collect::<String>();
        let name = names[function];
        let krate = (*function - 85) / 4096;
        writeln!(adapters, "#[export_name = {:?}] pub unsafe extern \"C\" fn export_{function}_{}({declarations}){result} {{ gen{krate:03}::{name}::{name}(context(){args}) }}", format!("engine_{export}"), export.replace('_', "x"))?;
    }
    let mut signatures: Vec<_> = canonical.iter().collect();
    signatures.sort_by_key(|(_, id)| **id);
    for ((params, results), id) in signatures {
        let params: Vec<String> = params.iter().map(|t| rust_type(t)).collect();
        let results: Vec<String> = results.iter().map(|t| rust_type(t)).collect();
        let result = if results.is_empty() { String::new() } else if results.len() == 1 { format!(" -> {}", results[0]) } else { format!(" -> ({})", results.join(", ")) };
        let declarations = params.iter().enumerate().map(|(i,t)| format!(", a{i}: {t}")).collect::<String>();
        let args = params.iter().enumerate().map(|(i,_)| format!(", a{i}")).collect::<String>();
        let types = params.iter().map(|t| format!(", {t}")).collect::<String>();
        writeln!(adapters, "#[no_mangle] pub unsafe extern \"C\" fn engine_dispatch_{id}(slot: i64{declarations}){result} {{ let ph = context(); let address = (*ph).table_entry_typed(slot, {id}); let function: unsafe extern \"C\" fn(*mut Phantom{types}){result} = core::mem::transmute(address); function(ph{args}) }}")?;
    }
    fs::write(out.join("adapters.rs"), adapters)?;
    fs::write(root.join("browser-exports.json"), serde_json::to_vec_pretty(&exports)?)?;
    println!("cargo:warning=linked module: {} functions, {} table entries, {} globals, memory {}..{} pages", function_types.len(), elements.len(), globals.len(), initial_pages, max_pages);
    Ok(())
}

fn main() { if let Err(e) = run() { panic!("module metadata: {e}"); } }
