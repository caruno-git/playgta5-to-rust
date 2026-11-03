import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from resource_readback_fixes import TARGETS

__author__ = 'carunodev @stfu'
ROOT = Path(__file__).resolve().parent.parent
RAW = ROOT / 'tools/testdata/readback'
LINKED = ROOT / 'linked/generated/gen000/src'
WAIT = 'f_000311_wasm_null_d3d_hasgpuid_waitreadback'
READBACK = 'f_000517_wasm_null_d3d_gpureadback_id3d11resource_ptr_unsigned_int_id3d11'

HARNESS = r'''
extern crate self as wasm_rt;
pub mod simd {}
#[path = r"__NUMERIC__"] mod numeric;
pub use numeric::*;
use std::collections::BTreeMap;
const OBJECT:i64=0x10000;
const SOURCE:i64=0x11000;
const STORAGE:i64=0x20000;
const SHADOW:i64=0x21000;
const PRIMARY_VTABLE:i64=0x400000;
const STORAGE_VTABLE:i64=0x410000;
pub struct Phantom {
    bytes:BTreeMap<i64,u8>,sp:i64,storage_offset:i64,pending:Option<(i64,i64)>,
    freed:Vec<i64>,deleted:Vec<i64>,destroyed:Vec<i32>,events:Vec<&'static str>,uaf:bool,completed:usize,
}
impl Phantom {
    fn read_u8(&self,a:i64)->u8{*self.bytes.get(&a).unwrap_or(&0)}
    fn read_u32(&self,a:i64)->u32{u32::from_le_bytes(std::array::from_fn(|i|self.read_u8(a+i as i64)))}
    fn read_u64(&self,a:i64)->u64{u64::from_le_bytes(std::array::from_fn(|i|self.read_u8(a+i as i64)))}
    fn read_v128(&self,a:i64)->u128{u128::from_le_bytes(std::array::from_fn(|i|self.read_u8(a+i as i64)))}
    fn write_u8(&mut self,a:i64,v:u8){self.bytes.insert(a,v);}
    fn write_u32(&mut self,a:i64,v:u32){for(i,b)in v.to_le_bytes().iter().enumerate(){self.write_u8(a+i as i64,*b);}}
    fn write_u64(&mut self,a:i64,v:u64){for(i,b)in v.to_le_bytes().iter().enumerate(){self.write_u8(a+i as i64,*b);}}
    fn write_v128(&mut self,a:i64,v:u128){for(i,b)in v.to_le_bytes().iter().enumerate(){self.write_u8(a+i as i64,*b);}}
    fn memory_fill(&mut self,a:i64,v:i32,n:i64){for i in 0..n{self.write_u8(a+i,v as u8);}}
    fn a_load_u32(&self,a:i64)->u32{self.read_u32(a)}
    fn a_load_u64(&self,a:i64)->u64{self.read_u64(a)}
    fn a_store_u32(&mut self,a:i64,v:i32){self.write_u32(a,v as u32);}
    fn a_store_u64(&mut self,a:i64,v:i64){self.write_u64(a,v as u64);}
    fn a_rmw_add_u64(&mut self,a:i64,v:i64)->i64{let old=self.read_u64(a);self.write_u64(a,old.wrapping_add(v as u64));old as i64}
    fn a_rmw_cmpxchg_u64(&mut self,a:i64,old:i64,new:i64)->i64{let current=self.read_u64(a);if current==old as u64{self.write_u64(a,new as u64);}current as i64}
    unsafe fn table_entry_typed(&self,index:i64,signature:u32)->*const(){match(index,signature){
        (1,0)=>storage as *const(),(2,0)=>storage_size as *const(),(3,14)=>layout as *const(),(4,3)=>resource_type as *const(),_=>panic!("Unexpected vtable call")}}
}
unsafe extern "C" fn storage(ph:*mut Phantom,interface:i64)->i64{assert_eq!(interface,OBJECT+16);OBJECT+(*ph).storage_offset}
unsafe extern "C" fn storage_size(_: *mut Phantom,interface:i64)->i64{assert_eq!(interface,OBJECT+16);64}
unsafe extern "C" fn layout(ph:*mut Phantom,out:i64,interface:i64,sub:i32){assert_eq!((interface,sub),(OBJECT+16,0));let p=&mut*ph;p.write_u64(out,0);p.write_u64(out+8,64);p.write_u32(out+16,16);p.write_u32(out+20,64);}
unsafe extern "C" fn resource_type(ph:*mut Phantom,object:i64,out:i64){assert_eq!(object,SOURCE);(*ph).write_u32(out,3);}
pub mod imp {pub fn imp_0_wasm_now_ms()->f64{10.0}}
pub mod syms {
    use super::*;
    pub unsafe fn f_091002_emscripten_builtin_free(ph:*mut Phantom,pointer:i64){
        if pointer==0{return;}let p=&mut*ph;assert!(!p.freed.contains(&pointer));p.freed.push(pointer);
        if pointer==STORAGE{p.events.push("free-storage");p.memory_fill(STORAGE,0x5a,16);}else{assert_eq!(pointer,SHADOW);p.events.push("free-shadow");}
    }
    pub unsafe fn f_000102_op_delete_void_ptr(ph:*mut Phantom,pointer:i64){assert_eq!(pointer,OBJECT);(*ph).deleted.push(pointer);}
    pub unsafe fn f_000556_wasm_gpu_destroyobject_unsigned_int(ph:*mut Phantom,id:i32){assert_eq!(id,19);(*ph).destroyed.push(id);}
    pub unsafe fn f_000311_wasm_null_d3d_hasgpuid_waitreadback(ph:*mut Phantom,pointer:i64){assert_eq!(pointer,OBJECT+24);wait::f_000311_wasm_null_d3d_hasgpuid_waitreadback(ph,pointer);}
    pub unsafe fn f_000137_wasm_wait_begin(_: *mut Phantom,_:i64,flag:i64){assert_eq!(flag,OBJECT+92);}
    pub unsafe fn f_000138_wasm_wait_end(_: *mut Phantom){}
    pub unsafe fn f_090735_do_futex_wait(ph:*mut Phantom,flag:i64,value:i32,ms:f64)->i32{
        let p=&mut*ph;assert_eq!((flag,value,ms),(OBJECT+92,1,5.0));let(address,pending_flag)=p.pending.take().unwrap();assert_eq!(flag,pending_flag);
        p.uaf|=p.freed.contains(&address);p.events.push("gpu-write");p.memory_fill(address,0xa9,64);p.write_u32(flag,0);p.completed+=1;0
    }
    pub unsafe fn f_091082_dynamic_cast(_: *mut Phantom,_:i64,_:i64,_:i64,_:i64)->i64{panic!("Expected seeded actual RTTI cache")}
    pub unsafe fn f_091003_emscripten_builtin_calloc(_: *mut Phantom,_:i64,_:i64)->i64{panic!("Expected existing storage allocation")}
    pub unsafe fn f_000550_wasm_gpu_emit_unsigned_int_unsigned_int_const_ptr_unsigned_int(ph:*mut Phantom,opcode:i32,payload:i64,words:i32){
        let p=&mut*ph;assert_eq!((opcode,words),(42,17));assert_eq!(p.read_u32(payload),29);
        let address=p.read_u64(payload+36) as i64;let flag=p.read_u64(payload+60) as i64;
        assert_eq!((address,flag),(STORAGE,OBJECT+92));assert_eq!(p.read_u32(flag),1);assert_eq!(p.read_u32(payload+44),16);
        assert_eq!(p.read_u32(payload+48),64);assert_eq!(p.read_u32(payload+56),64);p.pending=Some((address,flag));p.events.push("emit-readback");
    }
}
#[path="wait.rs"] mod wait;
#[path="readback.rs"] mod readback;
__MODULES__
fn context(offset:i64,has_storage:bool)->Phantom{
    let mut p=Phantom{bytes:BTreeMap::new(),sp:0x90000,storage_offset:offset,pending:None,freed:vec![],deleted:vec![],destroyed:vec![],events:vec![],uaf:false,completed:0};
    p.write_u64(OBJECT,PRIMARY_VTABLE as u64);p.write_u64(SOURCE,PRIMARY_VTABLE as u64);p.write_u64(OBJECT+16,STORAGE_VTABLE as u64);
    p.write_u32(OBJECT+32,19);p.write_u32(SOURCE+32,29);p.write_u64(OBJECT+offset,if has_storage{STORAGE as u64}else{0});p.write_u64(OBJECT+offset+8,if has_storage{64}else{0});
    p.write_u64(OBJECT+48,SHADOW as u64);p.write_u64(OBJECT+56,32);p.write_u64(8324208,if has_storage{64}else{0});p.write_u64(8324216,if has_storage{1}else{0});p.write_u64(8324232,32);
    let cache=(PRIMARY_VTABLE>>1)&4088;p.write_u64(8328368+cache,((PRIMARY_VTABLE<<16)|24) as u64);p.write_u64(8332464+cache,((PRIMARY_VTABLE<<16)|16) as u64);
    for(offset,index)in[(16,1),(24,2),(32,3)]{p.write_u64(STORAGE_VTABLE+offset,index);}
    p.write_u64(PRIMARY_VTABLE+56,4);p.memory_fill(STORAGE,0,64);p.memory_fill(STORAGE-8,0xcc,8);p.memory_fill(STORAGE+64,0xdd,8);p
}
fn check(p:&Phantom,pending:bool,has_storage:bool,deleting:bool){
    assert_eq!(p.uaf,pending&&!__FIXED__);assert_eq!(p.completed,if pending{1}else{0});assert!(p.pending.is_none());assert_eq!(p.read_u32(OBJECT+92),0);
    assert_eq!(p.freed,if has_storage{vec![STORAGE,SHADOW]}else{vec![SHADOW]});assert_eq!(p.deleted.len(),usize::from(deleting));assert_eq!(p.destroyed,vec![19]);
    assert_eq!(p.read_u64(8324208),0);assert_eq!(p.read_u64(8324216),0);assert_eq!(p.read_u64(8324232),0);assert_eq!(p.sp,0x90000);
    assert_eq!(p.read_u64(STORAGE-8),0xcccccccccccccccc);assert_eq!(p.read_u64(STORAGE+64),0xdddddddddddddddd);
    if pending{assert_eq!(p.events,if __FIXED__{vec!["emit-readback","gpu-write","free-storage","free-shadow"]}else{vec!["emit-readback","free-storage","gpu-write","free-shadow"]});}
    if has_storage{assert_eq!(p.read_u8(STORAGE),if p.uaf{0xa9}else{0x5a});}
}
fn main(){
    for(pending,has_storage)in[(true,true),(false,true),(false,false)]{
        __CASES__
    }
    println!("{{\"author\":\"carunodev @stfu\",\"fixed\":{},\"destructor_variants\":24,\"cases\":72,\"actual_readback_command_executed\":true,\"actual_wait_executed\":true,\"completion_in_futex_fixture\":true,\"writes_after_free\":{},\"live_gpu_run\":false}}",__FIXED__,if __FIXED__{0}else{24});
}
'''


@unittest.skipUnless(shutil.which('rustc'), 'rustc is required for actual translated resource lifetime execution')
class ResourceReadbackDeliveryTests(unittest.TestCase):
    def test_actual_readback_wait_and_all_destructors_preserve_storage_until_completion(self):
        flags = subprocess.BELOW_NORMAL_PRIORITY_CLASS | subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0
        reports = []
        with tempfile.TemporaryDirectory(prefix='playgta5-readback-lifetime-') as temporary:
            directory = Path(temporary)
            for variant, source_directory in [('before', RAW), ('fixed', LINKED)]:
                modules, cases = [], []
                for index, (filename, (storage_offset, adjustment, deleting)) in enumerate(TARGETS.items()):
                    source = (source_directory / filename).read_text(encoding='utf-8').replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).')
                    (directory / f'destructor{index}.rs').write_text(source, encoding='utf-8')
                    modules.append(f'#[path="destructor{index}.rs"] mod destructor{index};')
                    call = f'destructor{index}::{filename.removesuffix(".rs")}(&mut p,OBJECT+{adjustment})'
                    invocation = call + ';' if deleting else 'assert_eq!(' + call + ',OBJECT);'
                    cases.append('{let mut p=context(' + str(storage_offset) + ',has_storage);if pending{assert_eq!(unsafe{readback::' + READBACK + '(&mut p,OBJECT,0,SOURCE,0,0)},1);}unsafe{' + invocation + '}check(&p,pending,has_storage,' + str(deleting).lower() + ');}')
                for module, name in [('wait', WAIT), ('readback', READBACK)]:
                    (directory / f'{module}.rs').write_text((LINKED / (name + '.rs')).read_text(encoding='utf-8'), encoding='utf-8')
                source = HARNESS.replace('__NUMERIC__', str(ROOT / 'wasm-rt/src/numeric.rs')).replace('__MODULES__', '\n'.join(modules)).replace('__CASES__', '\n'.join(cases)).replace('__FIXED__', str(variant == 'fixed').lower())
                rust, binary = directory / 'harness.rs', directory / ('lifetime.exe' if os.name == 'nt' else 'lifetime')
                rust.write_text(source, encoding='utf-8')
                built = subprocess.run(['rustc', '--edition=2021', '-Awarnings', str(rust), '-o', str(binary)], capture_output=True, text=True, timeout=60, creationflags=flags)
                self.assertEqual(built.returncode, 0, built.stdout + built.stderr)
                run = subprocess.run([str(binary)], capture_output=True, text=True, timeout=15, creationflags=flags)
                self.assertEqual(run.returncode, 0, run.stdout + run.stderr)
                reports.append(json.loads(run.stdout))
        self.assertEqual([report['writes_after_free'] for report in reports], [24, 0])
        print(json.dumps({'resource_readback_lifetime': reports}, sort_keys=True))


if __name__ == '__main__':
    unittest.main()
