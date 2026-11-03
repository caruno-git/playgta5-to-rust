import ast
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from task_upload_fixes import TARGET, CALLEE, apply_task_upload_fixes

__author__ = 'carunodev @stfu'
TOOLS = Path(__file__).resolve().parent
ROOT = TOOLS.parent
RAW = TOOLS / 'testdata' / TARGET
LINKED = ROOT / 'linked/generated/gen001/src' / TARGET


def linked_text(text):
    return text.replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).')


class TaskUploadTests(unittest.TestCase):
    def setUp(self):
        self.raw = RAW.read_text(encoding='utf-8')

    def test_real_raw_source_only_three_registrations_added(self):
        fixed = apply_task_upload_fixes(TARGET, self.raw)
        added = [line for line in fixed.splitlines() if line not in self.raw.splitlines()]
        self.assertEqual([line.strip() for line in added], [
            'ph.write_u64((ph.g[28] as i64), 4095i64 as u64);',
            'ph.write_u64((ph.g[29] as i64), 4096i64 as u64);',
            'ph.write_u64(12834520i64, 4097i64 as u64);',
        ])
        self.assertEqual(fixed.replace('\n'.join(added) + '\n', '', 1), self.raw)

    def test_exact_idempotence_raw_and_linked(self):
        for original in [self.raw, linked_text(self.raw)]:
            fixed = apply_task_upload_fixes(TARGET, original)
            self.assertEqual(apply_task_upload_fixes(TARGET, fixed), fixed)

    def test_crlf_preserved(self):
        original = self.raw.replace('\n', '\r\n')
        fixed = apply_task_upload_fixes(TARGET, original)
        self.assertEqual(fixed.count('\r\n'), original.count('\r\n') + 3)
        self.assertNotIn('\n', fixed.replace('\r\n', ''))

    def test_other_function_unchanged(self):
        self.assertEqual(apply_task_upload_fixes('f_005102_cloth_update.rs', self.raw), self.raw)

    def test_rejects_signature_or_context_drift(self):
        changes = [
            ('mut l0: i64', 'mut l0: i32'),
            ('let mut l2: i64 = 0;', 'let mut l20: i64 = 0;'),
            ('l1 = (ph.sp.wrapping_add(-64i64));', 'l1 = (ph.sp.wrapping_add(-80i64));'),
            (f'{CALLEE}(ph, l2, l1)', f'{CALLEE}(ph, l1, l2)'),
            ('ph.sp = (l1.wrapping_sub(-64i64));', 'ph.sp = l1;'),
        ]
        for old, new in changes:
            with self.subTest(old=old):
                self.assertIn(old, self.raw)
                with self.assertRaises(ValueError):
                    apply_task_upload_fixes(TARGET, self.raw.replace(old, new, 1))
        with self.assertRaises(ValueError):
            apply_task_upload_fixes(TARGET, self.raw + self.raw)

    def test_rejects_partial_or_changed_callbacks(self):
        fixed = apply_task_upload_fixes(TARGET, self.raw)
        for bad in [fixed.replace('4096i64', '4999i64'),
                    fixed.replace('        ph.write_u64((ph.g[28] as i64), 4095i64 as u64);\n', ''),
                    fixed.replace('        ph.write_u64(12834520i64, 4097i64 as u64);\n', ''),
                    fixed.replace('g[28]', 'g[29]')]:
            with self.assertRaises(ValueError):
                apply_task_upload_fixes(TARGET, bad)

    def test_sibling_callback_setup_and_table_ids_match(self):
        sibling = (ROOT / 'linked/generated/gen001/src/f_005102_rage_clothmanager_updatetask_rage_systaskparameters_ref.rs').read_text(encoding='utf-8')
        fixed = apply_task_upload_fixes(TARGET, linked_text(self.raw))
        registrations = [line.strip() for line in fixed.splitlines() if any(value in line for value in ('4095i64', '4096i64', '4097i64'))]
        self.assertEqual(len(registrations), 3)
        for registration in registrations:
            self.assertIn(registration, sibling)

    def test_current_linked_source_already_fixed(self):
        text = LINKED.read_text(encoding='utf-8')
        self.assertEqual(apply_task_upload_fixes(TARGET, text), text)
        self.assertIn('g[29] as i64), 4096i64 as u64);', text)

    def test_generator_calls_fix_once(self):
        tree = ast.parse((TOOLS / 'link_engine.py').read_text(encoding='utf-8'))
        calls = [node for node in ast.walk(tree) if isinstance(node, ast.Call) and
                 isinstance(node.func, ast.Name) and node.func.id == 'apply_task_upload_fixes']
        self.assertEqual(len(calls), 1)
        self.assertEqual(ast.unparse(calls[0]), 'apply_task_upload_fixes(source.name, text)')

    @unittest.skipUnless(shutil.which('rustc'), 'rustc is required for actual task execution')
    def test_actual_translated_task_executes_with_callbacks_and_preserved_params(self):
        harness = r'''
extern crate self as wasm_rt;
pub mod simd {}
pub mod imp {}
pub struct Phantom { pub sp:i64, pub g:[u64;64], mem:Vec<u8>, calls:usize }
impl Phantom {
    fn read_u64(&self,a:i64)->u64 {u64::from_le_bytes(self.mem[a as usize..a as usize+8].try_into().unwrap())}
    fn read_u32(&self,a:i64)->u32 {u32::from_le_bytes(self.mem[a as usize..a as usize+4].try_into().unwrap())}
    fn write_u64(&mut self,a:i64,v:u64) {self.mem[a as usize..a as usize+8].copy_from_slice(&v.to_le_bytes());}
    fn write_u32(&mut self,a:i64,v:u32) {self.mem[a as usize..a as usize+4].copy_from_slice(&v.to_le_bytes());}
    fn write_v128(&mut self,a:i64,v:u128) {self.mem[a as usize..a as usize+16].copy_from_slice(&v.to_le_bytes());}
    fn read_splat(&self,a:i64,bits:u32)->u128 {assert_eq!(bits,32);self.read_u32(a) as u128 * 0x00000001000000010000000100000001u128}
    fn load_lane(&self,a:i64,v:u128,lane:u32,bits:u32)->u128 {assert_eq!(bits,32);let shift=lane*32;(v&!(0xffffffffu128<<shift))|((self.read_u32(a)as u128)<<shift)}
}
pub mod syms {
    use super::Phantom;
    pub unsafe fn __CALLEE__(p:*mut Phantom,manager:i64,params:i64) {
        let p=&mut*p;
        assert_eq!(manager,0x123456789);
        assert_eq!(params,p.sp);
        assert_eq!(p.read_u64(p.g[28] as i64),4095);
        assert_eq!(p.read_u64(p.g[29] as i64),4096);
        assert_eq!(p.read_u64(12834520),4097);
        assert_eq!(p.read_u64(params),0x1111222233334444);
        assert_eq!(p.read_u64(params+8),0x5555666677778888);
        assert_eq!(p.read_u32(params+16),0xaabbccdd);
        assert_eq!(p.read_u64(params+24),0x9999aaaabbbbcccc);
        assert_eq!(p.read_u32(params+32),0x3f000001);
        assert_eq!(p.read_u32(params+36),0x3f000002);
        assert_eq!(p.read_u32(params+40),0x3f000003);
        assert_eq!(p.read_u32(params+44),0x3f000001);
        assert_eq!(p.read_u32(params+48),0x76543210);
        assert_eq!(p.read_u32(params+52),0xfedcba98);
        p.calls+=1;
    }
}
#[path="task.rs"] mod task;
fn main() {
    let mut p=Phantom{sp:65536,g:[0;64],mem:vec![0;12834528],calls:0};
    p.g[28]=128;p.g[29]=136;
    p.write_u64(128,777);p.write_u64(136,888);p.write_u64(12834520,999);
    let task=1024i64;
    p.write_u64(task+8,0x123456789);
    p.write_u64(task+136,0x1111222233334444);p.write_u64(task+144,0x5555666677778888);
    p.write_u32(task+160,0xaabbccdd);p.write_u64(task+152,0x9999aaaabbbbcccc);
    p.write_u32(task+184,0x3f000001);p.write_u32(task+192,0x3f000002);p.write_u32(task+200,0x3f000003);
    p.write_u32(task+168,0x76543210);p.write_u32(task+176,0xfedcba98);
    unsafe {task::__TASK__(&mut p as *mut Phantom,task);}
    assert_eq!(p.calls,1);assert_eq!(p.sp,65536);
    println!("PASS actual translated cloth task: callbacks, parameter packing, stack restoration");
}
'''.replace('__CALLEE__', CALLEE).replace('__TASK__', TARGET.removesuffix('.rs'))
        fixed = apply_task_upload_fixes(TARGET, LINKED.read_text(encoding='utf-8'))
        with tempfile.TemporaryDirectory(prefix='playgta5-task-upload-') as directory:
            directory = Path(directory)
            (directory / 'task.rs').write_text(fixed, encoding='utf-8')
            source = directory / 'harness.rs'
            source.write_text(harness, encoding='utf-8')
            binary = directory / ('harness.exe' if os.name == 'nt' else 'harness')
            compiled = subprocess.run(['rustc', '--edition=2021', '-Awarnings', str(source), '-o', str(binary)],
                                      capture_output=True, text=True, timeout=120)
            self.assertEqual(compiled.returncode, 0, compiled.stdout + compiled.stderr)
            executed = subprocess.run([str(binary)], capture_output=True, text=True, timeout=15)
            self.assertEqual(executed.returncode, 0, executed.stdout + executed.stderr)
            self.assertIn('PASS actual translated cloth task', executed.stdout)


if __name__ == '__main__':
    unittest.main()
