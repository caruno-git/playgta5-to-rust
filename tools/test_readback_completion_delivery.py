import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from test_resource_readback_delivery import HARNESS, LINKED, RAW, READBACK, ROOT, WAIT

__author__ = 'carunodev @stfu'
DESTRUCTOR = 'f_000312_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture2d_d3'
MAIN = r'''
static CURRENT:std::sync::atomic::AtomicPtr<Phantom>=std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
pub fn trap(message:&str)->!{
    assert_eq!(message,"GPU readback incomplete after wait limit");
    let p=unsafe{&*CURRENT.load(std::sync::atomic::Ordering::SeqCst)};
    assert_eq!(p.wait_calls,12000);assert_eq!(p.read_u32(OBJECT+92),1);
    assert!(p.freed.is_empty()&&p.deleted.is_empty()&&p.destroyed.is_empty()&&p.pending.is_some());
    println!("{{\"fatal_before_free\":true,\"storage_retained\":true,\"ack_owner_retained\":true,\"late_write_after_free\":false}}");
    std::process::exit(73)
}
fn main(){
    let mut p=context(144,true);
    p.complete_at=std::env::args().nth(1).unwrap().parse().unwrap();
    CURRENT.store(&mut p,std::sync::atomic::Ordering::SeqCst);
    assert_eq!(unsafe{readback::f_000517_wasm_null_d3d_gpureadback_id3d11resource_ptr_unsigned_int_id3d11(&mut p,OBJECT,0,SOURCE,0,0)},1);
    unsafe{destructor::f_000312_wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11texture2d_d3(&mut p,OBJECT);}
    let pending=p.pending.is_some();
    if pending{
        assert_eq!(p.wait_calls,12000);assert!(p.freed.contains(&STORAGE));assert_eq!(p.deleted,vec![OBJECT]);
        p.complete_at=12001;unsafe{syms::f_090735_do_futex_wait(&mut p,OBJECT+92,1,5.0);}
        assert!(p.uaf);
    }else{assert!(!p.uaf);assert_eq!(p.wait_calls,p.complete_at);}
    println!("{{\"fatal_before_free\":false,\"wait_calls\":{},\"late_write_after_free\":{}}}",p.wait_calls,p.uaf);
}
'''


def once(source, old, new):
    if source.count(old) != 1:
        raise ValueError('Actual-source fixture drift')
    return source.replace(old, new, 1)


@unittest.skipUnless(shutil.which('rustc'), 'rustc required for actual readback completion execution')
class ReadbackCompletionDeliveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix='playgta5-readback-completion-')
        cls.addClassCleanup(cls.temporary.cleanup)
        directory = Path(cls.temporary.name)
        cls.executables = {}
        cls.flags = subprocess.BELOW_NORMAL_PRIORITY_CLASS | subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0
        source = HARNESS.split('fn main(){', 1)[0] + MAIN
        source = once(source, 'bytes:BTreeMap<i64,u8>,sp:i64,storage_offset:i64,pending:Option<(i64,i64)>,',
                      'bytes:BTreeMap<i64,u8>,sp:i64,storage_offset:i64,pending:Option<(i64,i64)>,wait_calls:usize,complete_at:usize,')
        source = once(source, 'storage_offset:offset,pending:None,freed:', 'storage_offset:offset,pending:None,wait_calls:0,complete_at:0,freed:')
        source = once(source, 'let(address,pending_flag)=p.pending.take().unwrap();',
                      'p.wait_calls+=1;if p.complete_at==0||p.wait_calls<p.complete_at{return -73;}let(address,pending_flag)=p.pending.take().unwrap();')
        source = source.replace('__NUMERIC__', str(ROOT / 'wasm-rt/src/numeric.rs')).replace('__MODULES__', '#[path="destructor.rs"] mod destructor;').replace('__FIXED__', 'true')
        (directory / 'harness.rs').write_text(source, encoding='utf-8')
        (directory / 'readback.rs').write_text((LINKED / (READBACK + '.rs')).read_text(encoding='utf-8'), encoding='utf-8')
        (directory / 'destructor.rs').write_text((LINKED / (DESTRUCTOR + '.rs')).read_text(encoding='utf-8'), encoding='utf-8')
        for variant, parent in [('before', RAW), ('fixed', LINKED)]:
            wait = (parent / (WAIT + '.rs')).read_text(encoding='utf-8').replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).')
            (directory / 'wait.rs').write_text(wait, encoding='utf-8')
            binary = directory / (variant + ('.exe' if os.name == 'nt' else ''))
            built = subprocess.run(['rustc', '--edition=2021', '-Awarnings', str(directory / 'harness.rs'), '-o', str(binary)],
                                   capture_output=True, text=True, timeout=60, creationflags=cls.flags)
            if built.returncode:
                raise AssertionError(built.stdout + built.stderr)
            cls.executables[variant] = binary

    def run_case(self, variant, completion):
        run = subprocess.run([str(self.executables[variant]), str(completion)], capture_output=True, text=True, timeout=10, creationflags=self.flags)
        return run.returncode, json.loads(run.stdout)

    def test_timeout_prevents_return_before_storage_and_ack_owner_are_freed(self):
        code, before = self.run_case('before', 0)
        self.assertEqual(code, 0)
        self.assertTrue(before['late_write_after_free'])
        code, fixed = self.run_case('fixed', 0)
        self.assertEqual(code, 73)
        self.assertTrue(fixed['fatal_before_free'] and fixed['storage_retained'] and fixed['ack_owner_retained'])
        self.assertFalse(fixed['late_write_after_free'])

    def test_first_and_last_attempt_completions_return_normally(self):
        for variant in self.executables:
            for completion in (1, 12000):
                with self.subTest(variant=variant, completion=completion):
                    code, result = self.run_case(variant, completion)
                    self.assertEqual(code, 0)
                    self.assertEqual(result['wait_calls'], completion)
                    self.assertFalse(result['fatal_before_free'] or result['late_write_after_free'])


if __name__ == '__main__':
    unittest.main()
