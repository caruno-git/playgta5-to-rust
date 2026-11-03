import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from minimap_mode_fixes import MODE_NAME, RESET_PATTERN, TARGET

__author__ = 'carunodev @stfu'
ROOT = Path(__file__).resolve().parent.parent
MODE = ROOT / 'linked/generated/gen000/src/f_000566_wasm_start_mode_now.rs'
SETTER = ROOT / 'linked/generated/gen012/src/f_051755_hud_commands_commandsetminimaphidefow_bool.rs'
INIT = ROOT / 'linked/generated/gen008/src' / TARGET
SYMBOLS = ROOT / 'linked/generated/gen008/src/syms.rs'
NUMERIC = ROOT / 'wasm-rt/src/numeric.rs'

HARNESS = r'''
extern crate self as wasm_rt;
pub mod simd {}
pub mod imp {}
#[path = r"__NUMERIC__"]
mod actual_numeric;
pub use actual_numeric::*;
use std::collections::BTreeMap;
const HIDE:i64=19520605;
const REINIT:i64=19520607;
const FOG:i64=19521072;
pub struct Phantom {
    words:BTreeMap<i64,u32>,bytes:BTreeMap<i64,u8>,writes:Vec<(i64,u8)>,arg_mode:i32,arg_calls:usize
}
impl Phantom {
    fn a_load_u32(&self,address:i64)->u32 {*self.words.get(&address).unwrap_or(&0)}
    fn read_u8(&self,address:i64)->u8 {*self.bytes.get(&address).unwrap_or(&0)}
    fn write_u8(&mut self,address:i64,value:u8){self.bytes.insert(address,value);self.writes.push((address,value));}
}
pub mod syms {
    use super::Phantom;
    __MODE_DECL__
    pub unsafe extern "C" fn f_000567_argstartmode(ph:*mut Phantom)->i32 {(*ph).arg_calls+=1;(*ph).arg_mode}
}
#[path="mode.rs"] mod actual_mode;
#[path="setter.rs"] mod actual_setter;
unsafe fn actual_reset_fragment_for_phase(ph:*mut Phantom,phase:i32) {
    if phase==8 {
        __FRAGMENT__
    }
}
fn context(committed:i32,arg:i32,fallback:i32,reinit:u8,hide:u8)->Phantom {
    let mut p=Phantom{words:BTreeMap::from([(6583880,committed as u32),(6583876,fallback as u32)]),bytes:BTreeMap::from([(REINIT,reinit),(HIDE,hide)]),writes:Vec::new(),arg_mode:arg,arg_calls:0};
    for offset in 0..16384 {p.bytes.insert(FOG+offset,((offset*37+11)&255) as u8);}
    p
}
fn check_fog(p:&Phantom){for offset in 0..16384 {assert_eq!(p.read_u8(FOG+offset),((offset*37+11)&255) as u8);}}
fn main(){
    let cases=[(0,0,2,0),(1,0,2,0),(2,0,1,1),(3,0,1,1),(-1,1,2,0),(-1,2,1,1),(-1,0,1,0),(-1,0,2,1),(-1,0,3,1),(-1,0,0,0),(-1,3,1,0)];
    for (committed,arg,fallback,wanted) in cases {
        let mut p=context(committed,arg,fallback,0,42);
        unsafe {actual_reset_fragment_for_phase(&mut p,8);}
        assert_eq!(p.read_u8(HIDE),wanted);
        assert_eq!(p.writes,vec![(19520604,0),(6974156,1),(HIDE,wanted)]);
        assert_eq!(p.arg_calls,if committed<0 {1}else{0});check_fog(&p);
    }
    for phase in [0,1,2,7,9] {
        let mut p=context(2,0,1,0,42);unsafe{actual_reset_fragment_for_phase(&mut p,phase);}
        assert_eq!(p.read_u8(HIDE),42);assert!(p.writes.is_empty());assert_eq!(p.arg_calls,0);check_fog(&p);
    }
    for reinit in [1,255] {for hide in [0,1] {
        let mut p=context(2,0,1,reinit,hide);unsafe{actual_reset_fragment_for_phase(&mut p,8);}
        assert_eq!(p.read_u8(HIDE),hide);assert!(p.writes.is_empty());assert_eq!(p.arg_calls,0);check_fog(&p);
    }}
    let mut p=context(2,0,1,0,0);unsafe{actual_reset_fragment_for_phase(&mut p,8);}
    assert_eq!(p.read_u8(HIDE),1);
    unsafe{actual_setter::f_051755_hud_commands_commandsetminimaphidefow_bool(&mut p,0);}
    assert_eq!(p.read_u8(HIDE),0);
    p.bytes.insert(REINIT,1);unsafe{actual_reset_fragment_for_phase(&mut p,8);}
    assert_eq!(p.read_u8(HIDE),0);
    unsafe{actual_setter::f_051755_hud_commands_commandsetminimaphidefow_bool(&mut p,1);}
    assert_eq!(p.read_u8(HIDE),1);check_fog(&p);
    println!("{{\"author\":\"carunodev @stfu\",\"fresh_mode_cases\":11,\"other_phase_cases\":5,\"reinit_cases\":4,\"script_override\":true,\"fog_canaries_unchanged\":true,\"actual_mode_now_executed\":true,\"actual_numeric_select_executed\":true,\"actual_script_setter_executed\":true,\"full_engine_init_executed\":false,\"argument_parser_executed\":false}}");
}
'''


@unittest.skipUnless(shutil.which('rustc'), 'rustc is required for the bounded actual mode/store fixture')
class NativeMinimapModeTests(unittest.TestCase):
    def test_actual_mode_now_store_fragment_reinit_and_later_script_override(self):
        text = INIT.read_text(encoding='utf-8')
        fragment = RESET_PATTERN.search(text)
        self.assertIsNotNone(fragment)
        declaration = next(line for line in SYMBOLS.read_text().splitlines() if 'pub fn ' + MODE_NAME + '(' in line)
        flags = subprocess.BELOW_NORMAL_PRIORITY_CLASS | subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0
        with tempfile.TemporaryDirectory(prefix='playgta5-minimap-mode-') as temporary:
            directory = Path(temporary).resolve()
            self.assertTrue(directory.is_relative_to(Path(tempfile.gettempdir()).resolve()))
            (directory / 'mode.rs').write_text(MODE.read_text(), encoding='utf-8')
            (directory / 'setter.rs').write_text(SETTER.read_text(), encoding='utf-8')
            source = HARNESS.replace('__NUMERIC__', str(NUMERIC.resolve())).replace('__MODE_DECL__', declaration).replace('__FRAGMENT__', fragment[0])
            rust = directory / 'harness.rs'
            rust.write_text(source, encoding='utf-8')
            binary = directory / ('mode-policy.exe' if os.name == 'nt' else 'mode-policy')
            compile_result = subprocess.run(['rustc', '--edition=2021', '-Awarnings', str(rust), '-o', str(binary)],
                                            capture_output=True, text=True, timeout=60, creationflags=flags)
            self.assertEqual(compile_result.returncode, 0, compile_result.stdout + compile_result.stderr)
            result = subprocess.run([str(binary)], capture_output=True, text=True, timeout=15, creationflags=flags)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            report = json.loads(result.stdout)
            self.assertEqual(report['fresh_mode_cases'], 11)
            self.assertEqual(report['other_phase_cases'], 5)
            self.assertEqual(report['reinit_cases'], 4)
            for field in ['script_override', 'fog_canaries_unchanged', 'actual_mode_now_executed', 'actual_numeric_select_executed', 'actual_script_setter_executed']:
                self.assertTrue(report[field], field)
            self.assertFalse(report['full_engine_init_executed'])
            self.assertFalse(report['argument_parser_executed'])
            print(json.dumps({'native_minimap_mode_policy': report}, sort_keys=True))


if __name__ == '__main__':
    unittest.main()
