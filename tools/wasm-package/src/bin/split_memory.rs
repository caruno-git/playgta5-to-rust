use std::{collections::BTreeMap, convert::Infallible, env, error::Error, fs, path::PathBuf};
use sha2::{Digest, Sha256};
use wasm_encoder::{reencode::{self, Reencode}, EntityType, ExportSection, ImportSection, Instruction as I, MemArg, MemoryType, Module, NameMap, NameSection};
use wasmparser::{Name, Operator, Parser, Payload, TypeRef, Validator, WasmFeatures};

struct Split {
    remap: Vec<Option<u32>>,
    helpers: BTreeMap<u32, String>,
    imports_after: u32,
    rewritten: u64,
    exports_before: u32,
    exports_after: u32,
    guest_min: u64,
    guest_max: u64,
}

fn memory(align: u32) -> MemArg { MemArg { offset: 0, align, memory_index: 1 } }

impl Split {
    fn intrinsic(&self, name: &str) -> I<'static> {
        macro_rules! atomics {
            ($($name:literal => $op:ident, $align:expr;)*) => {
                match name { $($name => return I::$op(memory($align)),)* _ => {} }
            };
        }
        atomics! {
            "a_load_u8" => I32AtomicLoad8U, 0; "a_store_u8" => I32AtomicStore8, 0;
            "a_rmw_add_u8" => I32AtomicRmw8AddU, 0; "a_rmw_sub_u8" => I32AtomicRmw8SubU, 0;
            "a_rmw_and_u8" => I32AtomicRmw8AndU, 0; "a_rmw_or_u8" => I32AtomicRmw8OrU, 0;
            "a_rmw_xor_u8" => I32AtomicRmw8XorU, 0; "a_rmw_xchg_u8" => I32AtomicRmw8XchgU, 0;
            "a_rmw_cmpxchg_u8" => I32AtomicRmw8CmpxchgU, 0;
            "a_load_u16" => I32AtomicLoad16U, 1; "a_store_u16" => I32AtomicStore16, 1;
            "a_rmw_add_u16" => I32AtomicRmw16AddU, 1; "a_rmw_sub_u16" => I32AtomicRmw16SubU, 1;
            "a_rmw_and_u16" => I32AtomicRmw16AndU, 1; "a_rmw_or_u16" => I32AtomicRmw16OrU, 1;
            "a_rmw_xor_u16" => I32AtomicRmw16XorU, 1; "a_rmw_xchg_u16" => I32AtomicRmw16XchgU, 1;
            "a_rmw_cmpxchg_u16" => I32AtomicRmw16CmpxchgU, 1;
            "a_load_u32" => I32AtomicLoad, 2; "a_store_u32" => I32AtomicStore, 2;
            "a_rmw_add_u32" => I32AtomicRmwAdd, 2; "a_rmw_sub_u32" => I32AtomicRmwSub, 2;
            "a_rmw_and_u32" => I32AtomicRmwAnd, 2; "a_rmw_or_u32" => I32AtomicRmwOr, 2;
            "a_rmw_xor_u32" => I32AtomicRmwXor, 2; "a_rmw_xchg_u32" => I32AtomicRmwXchg, 2;
            "a_rmw_cmpxchg_u32" => I32AtomicRmwCmpxchg, 2;
            "a_load_u64" => I64AtomicLoad, 3; "a_store_u64" => I64AtomicStore, 3;
            "a_rmw_add_u64" => I64AtomicRmwAdd, 3; "a_rmw_sub_u64" => I64AtomicRmwSub, 3;
            "a_rmw_and_u64" => I64AtomicRmwAnd, 3; "a_rmw_or_u64" => I64AtomicRmwOr, 3;
            "a_rmw_xor_u64" => I64AtomicRmwXor, 3; "a_rmw_xchg_u64" => I64AtomicRmwXchg, 3;
            "a_rmw_cmpxchg_u64" => I64AtomicRmwCmpxchg, 3;
        }
        match name {
            "read_u8" => I::I32Load8U(memory(0)), "read_i8" => I::I32Load8S(memory(0)),
            "read_u16" => I::I32Load16U(memory(0)), "read_i16" => I::I32Load16S(memory(0)),
            "read_u32" | "read_i32" => I::I32Load(memory(0)), "read_u64" => I::I64Load(memory(0)),
            "read_f32" => I::F32Load(memory(0)), "read_f64" => I::F64Load(memory(0)), "read_v128" => I::V128Load(memory(0)),
            "write_u8" | "write_i8" => I::I32Store8(memory(0)), "write_u16" | "write_i16" => I::I32Store16(memory(0)),
            "write_u32" | "write_i32" => I::I32Store(memory(0)), "write_u64" => I::I64Store(memory(0)),
            "write_f32" => I::F32Store(memory(0)), "write_f64" => I::F64Store(memory(0)), "write_v128" => I::V128Store(memory(0)),
            "memory_size" => I::MemorySize(1), "memory_grow" => I::MemoryGrow(1),
            "memory_copy" => I::MemoryCopy { src_mem: 1, dst_mem: 1 },
            "memory_init_copy" => I::MemoryCopy { src_mem: 0, dst_mem: 1 },
            "memory_fill" => I::MemoryFill(1),
            "a_wait32" => I::MemoryAtomicWait32(memory(2)), "a_notify" => I::MemoryAtomicNotify(memory(2)),
            _ => panic!("unknown guest intrinsic {name}"),
        }
    }
}

impl Reencode for Split {
    type Error = Infallible;
    fn function_index(&mut self, index: u32) -> Result<u32, reencode::Error<Self::Error>> {
        if (index as usize) < self.remap.len() {
            Ok(self.remap[index as usize].expect("guest intrinsic used as a function reference"))
        } else { Ok(index - self.remap.len() as u32 + self.imports_after) }
    }
    fn instruction<'a>(&mut self, instruction: Operator<'a>) -> Result<I<'a>, reencode::Error<Self::Error>> {
        if let Operator::Call { function_index } = instruction {
            if let Some(name) = self.helpers.get(&function_index) {
                self.rewritten += 1;
                return Ok(self.intrinsic(name));
            }
        }
        reencode::utils::instruction(self, instruction)
    }
    fn parse_import_section(&mut self, imports: &mut ImportSection, reader: wasmparser::ImportSectionReader<'_>) -> Result<(), reencode::Error<Self::Error>> {
        for import in reader {
            let import = import?;
            if import.module == "engine_intrinsics" { continue; }
            if let TypeRef::Memory(ty) = import.ty {
                imports.import("env", "rust_memory", EntityType::Memory(self.memory_type(ty)?));
            } else {
                imports.import(import.module, import.name, self.entity_type(import.ty)?);
            }
        }
        imports.import("env", "memory", EntityType::Memory(MemoryType { minimum: self.guest_min, maximum: Some(self.guest_max), memory64: true, shared: true, page_size_log2: None }));
        Ok(())
    }
    fn parse_export_section(&mut self, exports: &mut ExportSection, reader: wasmparser::ExportSectionReader<'_>) -> Result<(), reencode::Error<Self::Error>> {
        for export in reader {
            let export = export?;
            self.exports_before += 1;
            if !export.name.starts_with("engine_") && !matches!(export.name, "__stack_pointer" | "__indirect_function_table") { continue; }
            self.exports_after += 1;
            exports.export(export.name, self.export_kind(export.kind)?, self.external_index(export.kind, export.index)?);
        }
        Ok(())
    }
    fn parse_custom_name_subsection(&mut self, names: &mut NameSection, section: Name<'_>) -> Result<(), reencode::Error<Self::Error>> {
        if let Name::Function(map) = section {
            let mut output = NameMap::new();
            for name in map {
                let name = name?;
                if self.helpers.contains_key(&name.index) { continue; }
                output.append(self.function_index(name.index)?, name.name);
            }
            names.functions(&output);
            Ok(())
        } else if matches!(section, Name::Local(_) | Name::Label(_)) { Ok(()) }
        else { reencode::utils::parse_custom_name_subsection(self, names, section) }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let input = PathBuf::from(args.next().ok_or("expected input output")?);
    let output = PathBuf::from(args.next().ok_or("expected output")?);
    let bytes = fs::read(input)?;
    Validator::new_with_features(WasmFeatures::all()).validate_all(&bytes)?;
    let mut split = Split { remap: Vec::new(), helpers: BTreeMap::new(), imports_after: 0, rewritten: 0, exports_before: 0, exports_after: 0, guest_min: 49152, guest_max: 262144 };
    let mut rust_memory_pages = (0, 0);
    for payload in Parser::new(0).parse_all(&bytes) {
        if let Payload::ImportSection(reader) = payload? {
            for import in reader {
                let import = import?;
                if let TypeRef::Memory(memory) = import.ty { rust_memory_pages = (memory.initial, memory.maximum.unwrap_or(262144)); }
                if matches!(import.ty, TypeRef::Func(_)) {
                    let index = split.remap.len() as u32;
                    if import.module == "engine_intrinsics" {
                        split.helpers.insert(index, import.name.into());
                        split.remap.push(None);
                    } else {
                        split.remap.push(Some(split.imports_after));
                        split.imports_after += 1;
                    }
                }
            }
        }
    }
    if split.helpers.is_empty() { return Err("no guest intrinsics found".into()); }
    let mut module = Module::new();
    split.parse_core_module(&mut module, Parser::new(0), &bytes)?;
    let result = module.finish();
    Validator::new_with_features(WasmFeatures::all()).validate_all(&result)?;
    fs::write(&output, &result)?;
    let report = serde_json::json!({"author":"carunodev @stfu", "helpers_removed":split.helpers.len(), "guest_instructions":split.rewritten, "exports_before":split.exports_before, "exports_after":split.exports_after, "bytes":result.len(), "sha256":format!("{:x}",Sha256::digest(&result)), "game_memory_pages":[49152,262144], "rust_memory_pages":rust_memory_pages});
    fs::write(output.with_extension("memory.json"), serde_json::to_vec_pretty(&report)?)?;
    println!("PASS: {} guest instructions, {} helper imports removed, exports {} -> {}, {} bytes", split.rewritten, split.helpers.len(), split.exports_before, split.exports_after, result.len());
    Ok(())
}

fn main() { if let Err(e) = run() { eprintln!("split memory: {e}"); std::process::exit(1); } }

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::{CodeSection, ExportKind, Function, FunctionSection, TypeSection, ValType};

    #[test]
    fn rewritten_guest_access_is_isolated_from_rust_memory() {
        let mut input = Module::new();
        let mut types = TypeSection::new();
        types.ty().function([ValType::I64], [ValType::I32]);
        types.ty().function([ValType::I64, ValType::I32], []);
        input.section(&types);
        let mut imports = ImportSection::new();
        imports.import("env", "memory", EntityType::Memory(MemoryType { minimum: 2, maximum: Some(4), memory64: true, shared: true, page_size_log2: None }));
        imports.import("engine_intrinsics", "read_u32", EntityType::Function(0));
        imports.import("engine_intrinsics", "write_u32", EntityType::Function(1));
        input.section(&imports);
        let mut functions = FunctionSection::new(); functions.function(0); input.section(&functions);
        let mut exports = ExportSection::new(); exports.export("engine_test", ExportKind::Func, 2); input.section(&exports);
        let mut code = CodeSection::new();
        let mut function = Function::new([]);
        function.instruction(&I::LocalGet(0)).instruction(&I::I32Const(123)).instruction(&I::Call(1));
        function.instruction(&I::LocalGet(0)).instruction(&I::I32Const(456)).instruction(&I::I32Store(MemArg { offset: 0, align: 0, memory_index: 0 }));
        function.instruction(&I::LocalGet(0)).instruction(&I::Call(0)).instruction(&I::End);
        code.function(&function); input.section(&code);
        let bytes = input.finish();
        Validator::new_with_features(WasmFeatures::all()).validate_all(&bytes).unwrap();
        let mut split = Split { remap: vec![None, None], helpers: BTreeMap::from([(0, "read_u32".into()), (1, "write_u32".into())]), imports_after: 0, rewritten: 0, exports_before: 0, exports_after: 0, guest_min: 2, guest_max: 4 };
        let mut output = Module::new();
        split.parse_core_module(&mut output, Parser::new(0), &bytes).unwrap();
        let output = output.finish();
        Validator::new_with_features(WasmFeatures::all()).validate_all(&output).unwrap();
        assert_eq!(split.rewritten, 2);
        let directory = env::temp_dir().join(format!("engine-split-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let file = directory.join("test.wasm"); fs::write(&file, output).unwrap();
        let script = "const fs=require('node:fs');(async()=>{const rust_memory=new WebAssembly.Memory({initial:2n,maximum:4n,shared:true,address:'i64'}),memory=new WebAssembly.Memory({initial:2n,maximum:4n,shared:true,address:'i64'});const {instance}=await WebAssembly.instantiate(fs.readFileSync(process.argv[1]),{env:{rust_memory,memory}});if(instance.exports.engine_test(16n)!==123||new Uint32Array(rust_memory.buffer)[4]!==456||new Uint32Array(memory.buffer)[4]!==123)throw Error('memory isolation failed');console.log('separate memories PASS');})().catch(e=>{console.error(e);process.exitCode=1});";
        let result = std::process::Command::new("node").args(["-e", script]).arg(&file).output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        fs::remove_dir_all(directory).unwrap();
    }
}
