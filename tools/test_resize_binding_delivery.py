import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import unittest

from resize_binding_fixes import CLEAR_NAMES, RESET_PATTERN, TARGET

__author__ = 'carunodev @stfu'
ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / 'linked/generated/gen002/src'
NUMERIC = ROOT / 'wasm-rt/src/numeric.rs'
SETTERS = (
    'f_010332_ragedirect3ddevicecontext11_vssetconstantbuffers_unsigned_int_un',
    'f_010341_ragedirect3ddevicecontext11_pssetconstantbuffers_unsigned_int_un',
    'f_010396_ragedirect3ddevicecontext11_cssetconstantbuffers_unsigned_int_un',
)

HARNESS = r'''
extern crate self as wasm_rt;
pub mod simd {}
pub mod imp {}
#[path = r"__NUMERIC__"] mod numeric;
pub use numeric::*;
use std::collections::BTreeMap;
const WRAPPER:i64=0x10000;
const DEVICE:i64=0x20000;
const VTABLE:i64=0x30000;
const INPUT:i64=0x40000;
const DEAD:i64=0x50000;
const NEW_A:i64=0x51000;
const NEW_B:i64=0x52000;
const STAGES:[(i64,i64);6]=__STAGES__;
pub struct Phantom { bytes:BTreeMap<i64,u8>,g:[u64;64],sp:i64,forwarded:Vec<(usize,i32,Vec<u64>)>,underlying:[[u64;16];6] }
impl Phantom {
    fn read_u64(&self,a:i64)->u64 {u64::from_le_bytes(std::array::from_fn(|i|*self.bytes.get(&(a+i as i64)).unwrap_or(&0)))}
    fn write_u64(&mut self,a:i64,v:u64){for(i,b)in v.to_le_bytes().iter().enumerate(){self.bytes.insert(a+i as i64,*b);}}
    fn memory_fill(&mut self,a:i64,v:i32,n:i64){for i in 0..n{self.bytes.insert(a+i,v as u8);}}
    unsafe fn table_entry_typed(&self,index:i64,signature:u32)->*const(){
        assert_eq!(signature,43);match index {__TABLE__ _=>panic!("unexpected fixture vtable slot")}
    }
}
__SINKS__
__MODULES__
pub mod syms {use super::Phantom;__DECLARATIONS__}
unsafe fn patched_shutdown_fragment(ph:*mut Phantom){let mut l0:i64;let mut l2:i64;__FRAGMENT__}
fn context()->Phantom {
    let mut p=Phantom{bytes:BTreeMap::new(),g:[0;64],sp:0x90000,forwarded:Vec::new(),underlying:[[0xABCD;16];6]};
    p.g[1]=0x60000;p.write_u64(p.g[1] as i64+1160,WRAPPER as u64);p.write_u64(WRAPPER+8,DEVICE as u64);p.write_u64(DEVICE,VTABLE as u64);
    for(stage,(offset,method))in STAGES.iter().enumerate(){
        p.write_u64(VTABLE+method,(stage+1)as u64);
        for slot in 0..14{p.write_u64(WRAPPER+offset+slot*8,DEAD as u64);p.underlying[stage][slot as usize]=DEAD as u64;}
    }
    p.write_u64(WRAPPER+1656-8,0xFACE1234);p.write_u64(WRAPPER+2328,0xDECA5678);
    p.write_u64(DEAD,0x123456);p.write_u64(INPUT,NEW_A as u64);p.write_u64(INPUT+8,0);p.write_u64(INPUT+16,NEW_B as u64);p
}
fn canaries(p:&Phantom){assert_eq!(p.read_u64(WRAPPER+1656-8),0xFACE1234);assert_eq!(p.read_u64(WRAPPER+2328),0xDECA5678);assert_eq!(p.sp,0x90000);}
fn main(){
    let mut old=context();old.write_u64(DEAD,0xDEADDEAD);
    unsafe{set_vs::f_010332_ragedirect3ddevicecontext11_vssetconstantbuffers_unsigned_int_un(&mut old,WRAPPER,0,3,INPUT);}
    let old_forward=old.forwarded.last().unwrap();assert_eq!(old_forward.1,0);assert_eq!(old_forward.2,vec![NEW_A as u64,DEAD as u64,NEW_B as u64]);
    assert_eq!(old.read_u64(DEAD),0xDEADDEAD);canaries(&old);
    let setters:[unsafe extern "C" fn(*mut Phantom,i64,i32,i32,i64);3]=[
      set_vs::f_010332_ragedirect3ddevicecontext11_vssetconstantbuffers_unsigned_int_un,
      set_ps::f_010341_ragedirect3ddevicecontext11_pssetconstantbuffers_unsigned_int_un,
      set_cs::f_010396_ragedirect3ddevicecontext11_cssetconstantbuffers_unsigned_int_un];
    for setter in setters {
        let mut p=context();unsafe{patched_shutdown_fragment(&mut p);}
        assert_eq!(p.forwarded.len(),6);
        for(stage,(offset,_))in STAGES.iter().enumerate(){for slot in 0..14{assert_eq!(p.read_u64(WRAPPER+offset+slot*8),0);assert_eq!(p.underlying[stage][slot as usize],0);}
            assert_eq!(p.underlying[stage][14],0xABCD);assert_eq!(p.underlying[stage][15],0xABCD);}
        assert_eq!(p.read_u64(DEAD),0x123456);canaries(&p);
        p.write_u64(DEAD,0xDEADDEAD);unsafe{setter(&mut p,WRAPPER,0,3,INPUT);}
        assert_eq!(p.forwarded.last().unwrap().2,vec![NEW_A as u64,0,NEW_B as u64]);
        assert!(!p.forwarded.iter().any(|event|event.2.contains(&(DEAD as u64))));
        let calls=p.forwarded.len();unsafe{setter(&mut p,WRAPPER,0,3,INPUT);}assert_eq!(p.forwarded.len(),calls);canaries(&p);
        unsafe{patched_shutdown_fragment(&mut p);patched_shutdown_fragment(&mut p);}canaries(&p);
    }
    println!("{{\"author\":\"carunodev @stfu\",\"actual_stage_setters\":3,\"actual_clear_methods\":6,\"old_freed_hole_forwarded\":true,\"new_hole_is_zero\":true,\"native_and_wrapper_slots_cleared\":true,\"canaries_unchanged\":true,\"clear_does_not_release_resource\":true,\"dynamic_cast_executed\":false,\"full_shutdown_executed\":false}}");
}
'''


@unittest.skipUnless(shutil.which('rustc'), 'rustc is required for the actual translated CB wrapper fixture')
class ResizeBindingDeliveryTests(unittest.TestCase):
    def test_actual_setters_and_clear_helpers_remove_freed_hole_before_rebind(self):
        shutdown = (SOURCE / TARGET).read_text(encoding='utf-8')
        reset = RESET_PATTERN.search(shutdown)
        self.assertIsNotNone(reset)
        self.assertEqual(reset['calls'].count('syms::'), 6)
        stages, modules, declarations, sinks, table = [], [], [], [], []
        for index, name in enumerate(CLEAR_NAMES):
            text = (SOURCE / (name + '.rs')).read_text(encoding='utf-8')
            offset = int(re.search(r'l1 = \(l0\.wrapping_add\((\d+)i64\)\);', text)[1])
            method = int(re.search(r'read_u64\(t2 \+ (\d+)\)', text)[1])
            stages.append((offset, method))
            modules.append(f'#[path="clear{index}.rs"] mod clear{index};')
            declarations.append(f'extern "C" {{pub fn {name}(ph:*mut Phantom,a0:i64);}}')
            table.append(f'{index + 1}=>sink{index} as *const (),')
            sinks.append(f'''unsafe extern "C" fn sink{index}(ph:*mut Phantom,_device:i64,start:i32,count:i32,array:i64){{
                let p=&mut *ph;let values:Vec<u64>=(0..count).map(|i|p.read_u64(array+i as i64*8)).collect();
                for(i,value)in values.iter().enumerate(){{p.underlying[{index}][start as usize+i]=*value;}}
                p.forwarded.push(({index},start,values));
            }}''')
        self.assertEqual(sorted(offset for offset, _ in stages), list(range(1656, 2328, 112)))
        flags = subprocess.BELOW_NORMAL_PRIORITY_CLASS | subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0
        with tempfile.TemporaryDirectory(prefix='playgta5-cb-reset-') as temporary:
            directory = Path(temporary).resolve()
            self.assertTrue(directory.is_relative_to(Path(tempfile.gettempdir()).resolve()))
            for index, name in enumerate(CLEAR_NAMES):
                (directory / f'clear{index}.rs').write_text((SOURCE / (name + '.rs')).read_text(encoding='utf-8'), encoding='utf-8')
            for stage, name in zip(['vs', 'ps', 'cs'], SETTERS):
                text = (SOURCE / (name + '.rs')).read_text(encoding='utf-8')
                self.assertNotIn('simd::', text)
                (directory / f'set_{stage}.rs').write_text(text, encoding='utf-8')
                modules.append(f'#[path="set_{stage}.rs"] mod set_{stage};')
            source = HARNESS.replace('__NUMERIC__', str(NUMERIC.resolve())).replace('__STAGES__', '[' + ','.join(f'({a},{b})' for a, b in stages) + ']')
            source = source.replace('__MODULES__', '\n'.join(modules)).replace('__DECLARATIONS__', '\n'.join(declarations))
            source = source.replace('__SINKS__', '\n'.join(sinks)).replace('__TABLE__', ''.join(table)).replace('__FRAGMENT__', reset['prefix'] + reset['calls'])
            rust, binary = directory / 'harness.rs', directory / ('cb-reset.exe' if os.name == 'nt' else 'cb-reset')
            rust.write_text(source, encoding='utf-8')
            compiled = subprocess.run(['rustc', '--edition=2021', '-Awarnings', str(rust), '-o', str(binary)], capture_output=True, text=True, timeout=60, creationflags=flags)
            self.assertEqual(compiled.returncode, 0, compiled.stdout + compiled.stderr)
            result = subprocess.run([str(binary)], capture_output=True, text=True, timeout=15, creationflags=flags)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            report = json.loads(result.stdout)
            self.assertTrue(report['old_freed_hole_forwarded'])
            self.assertTrue(report['new_hole_is_zero'])
            self.assertFalse(report['dynamic_cast_executed'])
            print(json.dumps({'actual_cb_reset_delivery': report}, sort_keys=True))


if __name__ == '__main__':
    unittest.main()
