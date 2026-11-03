import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from task_upload_fixes import CALLEE, TARGET

__author__ = 'carunodev @stfu'
ROOT = Path(__file__).resolve().parent.parent
RAW = ROOT / 'tools/testdata' / TARGET
FIXED = ROOT / 'linked/generated/gen001/src' / TARGET
UNLOCK_NAME = 'f_009772_rage_grcbufferd3d11_unlock_internal_bool_const'
UNLOCK = ROOT / 'linked/generated/gen002/src' / (UNLOCK_NAME + '.rs')

HARNESS = r'''
extern crate self as wasm_rt;
pub mod simd {}
pub mod imp {}
const TLS:i64=1024;
const TASK:i64=8192;
const BUFFER:i64=16384;
const CPU:i64=32768;
const SOURCE:i64=131072;
const CPU_BYTES:usize=256;
const MANAGER:i64=0x123456789;
const INITIAL_SP:i64=65536;
#[derive(Debug)]
struct Upload {buffer:i64,source:i64,offset:i32,length:i32,owned:i32,bytes:Vec<u8>}
pub struct Phantom {
    pub sp:i64,pub g:[u64;64],mem:Vec<u8>,warnings:Vec<(i64,i64,i64)>,uploads:Vec<Upload>,
    calls:usize,table_calls:Vec<(i64,u32)>
}
impl Phantom {
    fn read_u16(&self,a:i64)->u16 {u16::from_le_bytes(self.mem[a as usize..a as usize+2].try_into().unwrap())}
    fn read_u32(&self,a:i64)->u32 {u32::from_le_bytes(self.mem[a as usize..a as usize+4].try_into().unwrap())}
    fn read_u64(&self,a:i64)->u64 {u64::from_le_bytes(self.mem[a as usize..a as usize+8].try_into().unwrap())}
    fn write_u16(&mut self,a:i64,v:u16) {self.mem[a as usize..a as usize+2].copy_from_slice(&v.to_le_bytes());}
    fn write_u32(&mut self,a:i64,v:u32) {self.mem[a as usize..a as usize+4].copy_from_slice(&v.to_le_bytes());}
    fn write_u64(&mut self,a:i64,v:u64) {self.mem[a as usize..a as usize+8].copy_from_slice(&v.to_le_bytes());}
    fn write_v128(&mut self,a:i64,v:u128) {self.mem[a as usize..a as usize+16].copy_from_slice(&v.to_le_bytes());}
    fn read_splat(&self,a:i64,bits:u32)->u128 {assert_eq!(bits,32);self.read_u32(a) as u128*0x00000001000000010000000100000001u128}
    fn load_lane(&self,a:i64,v:u128,lane:u32,bits:u32)->u128 {assert_eq!(bits,32);let shift=lane*32;(v&!(0xffffffffu128<<shift))|((self.read_u32(a) as u128)<<shift)}
    fn memory_copy(&mut self,dst:i64,src:i64,len:i64) {self.mem.copy_within(src as usize..(src+len) as usize,dst as usize);}
    fn table_entry_typed(&mut self,slot:i64,signature:u32)->*const () {
        self.table_calls.push((slot,signature));
        assert_eq!((slot,signature),(4096,40));
        enqueue_upload as *const ()
    }
}
unsafe extern "C" fn enqueue_upload(p:*mut Phantom,buffer:i64,source:i64,offset:i32,length:i32,owned:i32) {
    let p=&mut*p;
    assert_eq!(buffer,BUFFER); assert_eq!(source,CPU); assert_eq!(owned,0);
    assert_eq!(p.read_u64(TLS+1160),0);
    let start=(source+offset as i64) as usize;
    let bytes=p.mem[start..start+length as usize].to_vec();
    p.uploads.push(Upload{buffer,source,offset,length,owned,bytes});
}
pub mod syms {
    use super::*;
    pub unsafe fn __CALLEE__(p:*mut Phantom,manager:i64,params:i64) {
        let context=&mut*p;
        assert_eq!(manager,MANAGER); assert_eq!(params,context.sp);
        let buffer=context.read_u64(params) as i64;
        let source=context.read_u64(params+8) as i64;
        let length=context.read_u32(params+16) as i64;
        let offset=context.read_u32(params+48) as i64;
        assert_eq!(buffer,BUFFER); assert_eq!(source,SOURCE);
        assert_eq!(context.read_u16(buffer+24),0x11a);
        assert_eq!(context.read_u64(TLS+1160),0);
        assert_eq!(context.read_u32(buffer+16) as i64,offset);
        assert_eq!(context.read_u32(buffer+20) as i64,length);
        let cpu=context.read_u64(buffer+8) as i64;
        context.memory_copy(cpu+offset,source,length);
        context.calls+=1;
        unlock::__UNLOCK__(p,buffer,0);
    }
    pub unsafe fn f_010146_rage_grcbufferd3d11_updategpucopy_internal_void_ptr_unsigned_int(_: *mut Phantom,_:i64,_:i64,_:i32,_:i32) {
        panic!("Unexpected immediate GPU update instead of queued task delivery");
    }
    pub unsafe fn f_000121_rage_sysipclockmutexcommand_rage_sysipcmutextag_ptr(_: *mut Phantom,_:i64)->i32 {
        panic!("Unexpected mutex lock in sync3 task path");
    }
    pub unsafe fn f_000122_rage_sysipcunlockmutex_rage_sysipcmutextag_ptr(_: *mut Phantom,_:i64)->i32 {
        panic!("Unexpected mutex unlock in sync3 task path");
    }
    pub unsafe fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(p:*mut Phantom,channel:i64,message:i64,args:i64) {
        assert_eq!((channel,message,args),(12374176,4063774,0));
        (*p).warnings.push((channel,message,args));
    }
}
#[path="task.rs"] mod task;
#[path="unlock.rs"] mod unlock;
fn hex(bytes:&[u8])->String {bytes.iter().map(|value|format!("{value:02x}")).collect()}
fn main() {
    let args:Vec<String>=std::env::args().collect();
    let offset:usize=args[1].parse().unwrap();
    let length:usize=args[2].parse().unwrap();
    assert!(length>0 && offset+length<=CPU_BYTES);
    let mut p=Phantom{sp:INITIAL_SP,g:[0;64],mem:vec![0;12834528],warnings:vec![],uploads:vec![],calls:0,table_calls:vec![]};
    p.g[1]=TLS as u64; p.g[28]=(TLS+2664) as u64; p.g[29]=(TLS+2656) as u64;
    p.mem[CPU as usize..CPU as usize+CPU_BYTES].fill(0xa5);
    let payload:Vec<u8>=(0..length).map(|index|((index*37+11)&255) as u8).collect();
    p.mem[SOURCE as usize..SOURCE as usize+length].copy_from_slice(&payload);
    p.write_u64(BUFFER,0x20000); p.write_u64(BUFFER+8,CPU as u64);
    p.write_u32(BUFFER+16,offset as u32); p.write_u32(BUFFER+20,length as u32); p.write_u16(BUFFER+24,0x11a);
    p.write_u64(TASK+8,MANAGER as u64); p.write_u64(TASK+136,BUFFER as u64); p.write_u64(TASK+144,SOURCE as u64);
    p.write_u32(TASK+160,length as u32); p.write_u32(TASK+168,offset as u32);
    unsafe {task::__TASK__(&mut p,TASK);}
    let cpu=&p.mem[CPU as usize..CPU as usize+CPU_BYTES];
    assert_eq!(&cpu[offset..offset+length],payload);
    assert!(cpu[..offset].iter().chain(cpu[offset+length..].iter()).all(|value|*value==0xa5));
    assert_eq!(p.calls,1); assert_eq!(p.sp,INITIAL_SP); assert_eq!(p.read_u64(TLS+1160),0);
    assert_eq!(p.read_u16(BUFFER+24),0x11a); assert_eq!(p.read_u64(BUFFER),0x20000);
    let callback_values=[p.read_u64(TLS+2664),p.read_u64(TLS+2656),p.read_u64(12834520)];
    if __FIXED__ {
        assert_eq!(p.warnings.len(),0); assert_eq!(p.uploads.len(),1);
        assert_eq!(callback_values,[4095,4096,4097]); assert_eq!(p.table_calls,vec![(4096,40)]);
        let upload=&p.uploads[0];
        assert_eq!(upload.offset,offset as i32); assert_eq!(upload.length,length as i32); assert_eq!(upload.bytes,payload);
    } else {
        assert_eq!(p.warnings.len(),1); assert!(p.uploads.is_empty()); assert!(p.table_calls.is_empty());
        assert_eq!(callback_values,[0,0,0]);
    }
    let queued=match p.uploads.first() {
        Some(value)=>format!("{{\"buffer\":{},\"source\":{},\"offset\":{},\"length\":{},\"owned\":{},\"bytes_hex\":\"{}\"}}",value.buffer,value.source,value.offset,value.length,value.owned,hex(&value.bytes)),
        None=>"null".to_string()
    };
    println!("{{\"variant\":\"__VARIANT__\",\"cpu_changed\":true,\"guard_bytes_intact\":true,\"stack_restored\":true,\"context_null\":true,\"queued_uploads\":{},\"warnings\":{},\"callback_values\":{:?},\"table_calls\":{},\"queue\":{}}}",
        p.uploads.len(),p.warnings.len(),callback_values,p.table_calls.len(),queued);
}
'''


@unittest.skipUnless(shutil.which('rustc'), 'rustc is required for actual task and unlock execution')
class TaskUploadDeliveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix='playgta5-task-delivery-')
        cls.addClassCleanup(cls.temp.cleanup)
        cls.directory = Path(cls.temp.name)
        raw = RAW.read_text(encoding='utf-8').replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).')
        fixed = FIXED.read_text(encoding='utf-8')
        unlock = UNLOCK.read_text(encoding='utf-8')
        cls.sources = {'raw': raw, 'fixed': fixed}
        cls.source_hashes = {name: hashlib.sha256(text.encode()).hexdigest() for name, text in cls.sources.items()}
        cls.source_hashes['unlock'] = hashlib.sha256(unlock.encode()).hexdigest()
        cls.binaries = {}
        for variant, task in cls.sources.items():
            directory = cls.directory / variant
            directory.mkdir()
            (directory / 'task.rs').write_text(task, encoding='utf-8')
            (directory / 'unlock.rs').write_text(unlock, encoding='utf-8')
            harness = HARNESS.replace('__CALLEE__', CALLEE).replace('__TASK__', TARGET.removesuffix('.rs'))
            harness = harness.replace('__UNLOCK__', UNLOCK_NAME).replace('__FIXED__', str(variant == 'fixed').lower()).replace('__VARIANT__', variant)
            source = directory / 'harness.rs'
            source.write_text(harness, encoding='utf-8')
            binary = directory / ('delivery.exe' if os.name == 'nt' else 'delivery')
            compiled = subprocess.run(['rustc', '--edition=2021', '-Awarnings', str(source), '-o', str(binary)],
                                      capture_output=True, text=True, timeout=120)
            if compiled.returncode:
                raise AssertionError(variant + ' native fixture compilation failed:\n' + compiled.stdout + compiled.stderr)
            cls.binaries[variant] = binary

    def execute(self, variant, offset=24, length=48):
        completed = subprocess.run([str(self.binaries[variant]), str(offset), str(length)], capture_output=True, text=True, timeout=15)
        self.assertEqual(completed.returncode, 0, completed.stdout + completed.stderr)
        report = json.loads(completed.stdout)
        self.assertEqual(report['variant'], variant)
        for field in ['cpu_changed', 'guard_bytes_intact', 'stack_restored', 'context_null']:
            self.assertTrue(report[field], field)
        return report

    def test_original_task_changes_cpu_but_drops_queued_upload_and_warns(self):
        report = self.execute('raw')
        self.assertEqual(report['queued_uploads'], 0)
        self.assertEqual(report['warnings'], 1)
        self.assertEqual(report['callback_values'], [0, 0, 0])
        self.assertEqual(report['table_calls'], 0)
        self.assertIsNone(report['queue'])
        print(json.dumps({'author': __author__, 'native_delivery': report}, sort_keys=True))

    def test_current_fixed_task_queues_exact_cpu_bytes_offset_and_length(self):
        report = self.execute('fixed')
        self.assertEqual(report['queued_uploads'], 1)
        self.assertEqual(report['warnings'], 0)
        self.assertEqual(report['queue'], {'buffer': 16384, 'source': 32768, 'offset': 24, 'length': 48, 'owned': 0,
                                         'bytes_hex': bytes((index * 37 + 11) & 255 for index in range(48)).hex()})
        print(json.dumps({'author': __author__, 'native_delivery': report, 'source_sha256': self.source_hashes}, sort_keys=True))

    def test_callback_registration_uses_tls_upload_slot_and_signature_40(self):
        report = self.execute('fixed')
        self.assertEqual(report['callback_values'], [4095, 4096, 4097])
        self.assertEqual(report['table_calls'], 1)
        self.assertEqual(report['queue']['owned'], 0)

    def test_fixed_delivery_preserves_zero_and_nonzero_subranges(self):
        for offset, length in [(0, 16), (64, 92), (208, 48)]:
            with self.subTest(offset=offset, length=length):
                report = self.execute('fixed', offset, length)
                self.assertEqual(report['warnings'], 0)
                self.assertEqual(report['queue']['offset'], offset)
                self.assertEqual(report['queue']['length'], length)
                self.assertEqual(report['queue']['bytes_hex'], bytes((index * 37 + 11) & 255 for index in range(length)).hex())


if __name__ == '__main__':
    unittest.main()
