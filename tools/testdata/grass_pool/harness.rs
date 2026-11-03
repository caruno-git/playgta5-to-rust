extern crate self as wasm_rt;
pub mod simd {}
pub mod imp {}
#[path = r"__NUMERIC__"] mod numeric;
pub use numeric::*;
#[path = "pool.rs"] mod pool;
use std::collections::{BTreeMap,BTreeSet,VecDeque};
const AUTHOR:&str="carunodev @stfu";
const FREE:i64=23333272;
const USED:i64=23333296;
const LAST_FRAME:i64=23333368;
const FRAME:i64=12321196;
pub struct Phantom{bytes:BTreeMap<i64,u8>,sp:i64,nodes:BTreeSet<i64>,next:i64,allocations:usize,warnings:usize}
impl Phantom{
    fn read_u8(&self,address:i64)->u8{*self.bytes.get(&address).unwrap_or(&0)}
    fn read_u32(&self,address:i64)->u32{u32::from_le_bytes(std::array::from_fn(|i|self.read_u8(address+i as i64)))}
    fn read_u64(&self,address:i64)->u64{u64::from_le_bytes(std::array::from_fn(|i|self.read_u8(address+i as i64)))}
    fn read_v128(&self,address:i64)->u128{u128::from_le_bytes(std::array::from_fn(|i|self.read_u8(address+i as i64)))}
    fn write_u32(&mut self,address:i64,value:u32){for(i,byte)in value.to_le_bytes().iter().enumerate(){self.bytes.insert(address+i as i64,*byte);}}
    fn write_u64(&mut self,address:i64,value:u64){for(i,byte)in value.to_le_bytes().iter().enumerate(){self.bytes.insert(address+i as i64,*byte);}}
    fn write_v128(&mut self,address:i64,value:u128){for(i,byte)in value.to_le_bytes().iter().enumerate(){self.bytes.insert(address+i as i64,*byte);}}
}
pub mod syms{
    use super::*;
    pub unsafe fn f_001557_rage_diagwarningf_rage_diagchannel_const_ref_char_const_ptr(ph:*mut Phantom,_:i64,_:i64,_:i64){(*ph).warnings+=1;}
    pub unsafe fn f_010148_rage_grcbufferd3d11resource_grcbufferd3d11resource(ph:*mut Phantom,address:i64)->i64{
        let p=&mut*ph;p.write_v128(address,0);p.write_v128(address+16,0);p.write_v128(address+32,0);p.write_u64(address+48,0);address
    }
    pub unsafe fn f_000100_op_new_unsigned_long(ph:*mut Phantom,size:i64)->i64{
        assert_eq!(size,72);let p=&mut*ph;let node=p.next;p.next+=72;p.allocations+=1;assert!(p.nodes.insert(node));node
    }
    pub unsafe fn f_010149_rage_grcbufferd3d11resource_dtorgrcbufferd3d11resource(_: *mut Phantom,address:i64)->i64{address}
    pub unsafe fn f_010150_rage_grcbufferd3d11resource_initialise_unsigned_int_unsigned_int(_: *mut Phantom,_:i64,count:i32,stride:i32,kind:i32,data:i64,flags:i32){assert_eq!((count,stride,kind,data,flags),(4096,4,3,0,2));}
}
fn context(count:usize,frame:u32)->Phantom{
    assert!(count>0);
    let mut p=Phantom{bytes:BTreeMap::new(),sp:0x80000,nodes:BTreeSet::new(),next:0x20000000,allocations:0,warnings:0};
    let nodes:Vec<i64>=(0..count).map(|i|0x10000000+i as i64*72).collect();
    for(i,node)in nodes.iter().enumerate(){p.nodes.insert(*node);p.write_u64(*node,if i+1<count{nodes[i+1]}else{FREE}as u64);p.write_u64(*node+8,if i>0{nodes[i-1]}else{FREE}as u64);}
    p.write_u64(FREE,nodes[0]as u64);p.write_u64(FREE+8,nodes[count-1]as u64);p.write_u64(FREE+16,count as u64);
    for index in 0..3{let head=USED+index*24;p.write_u64(head,head as u64);p.write_u64(head+8,head as u64);p.write_u64(head+16,0);}
    p.write_u32(FRAME,frame);p.write_u32(LAST_FRAME,frame);p.write_u32(FREE-8,count as u32);
    p.write_u64(FREE-16,0x1122334455667788);p.write_u64(LAST_FRAME+8,0x8877665544332211);p
}
fn check_lists(p:&Phantom){
    let mut seen=BTreeSet::new();
    for head in [FREE,USED,USED+24,USED+48]{
        let mut node=p.read_u64(head)as i64;let mut previous=head;let mut count=0;
        while node!=head{assert!(p.nodes.contains(&node));assert!(seen.insert(node));assert_eq!(p.read_u64(node+8)as i64,previous);previous=node;node=p.read_u64(node)as i64;count+=1;assert!(count<=p.nodes.len());}
        assert_eq!(p.read_u64(head+8)as i64,previous);assert_eq!(p.read_u64(head+16),count as u64);
    }
    assert_eq!(seen,p.nodes);assert_eq!(p.sp,0x80000);assert_eq!(p.read_u64(FREE-16),0x1122334455667788);assert_eq!(p.read_u64(LAST_FRAME+8),0x8877665544332211);
}
fn run_case(name:&str,count:usize,per_frame:usize,frames:Vec<u32>){
    let mut p=context(count,frames[0]);let mut history=BTreeMap::<i64,u32>::new();let mut pending=VecDeque::<(u32,i64,u64)>::new();
    let mut minimum_gap=u32::MAX;let mut checksum=0u64;let mut calls=0;
    for frame in &frames{
        while pending.front().map(|(then,_,_)|frame.wrapping_sub(*then)>=3).unwrap_or(false){let(_,resource,value)=pending.pop_front().unwrap();assert_eq!(p.read_u64(resource+8),value);checksum=checksum.wrapping_mul(16777619)^value;}
        p.write_u32(FRAME,*frame);
        let mut used=BTreeSet::new();
        for item in 0..per_frame{
            let resource=unsafe{pool::f_046589_ebstatic_preallocatedresourcepool_ebstatic_preallocatedappendbuf(&mut p)};
            assert!(used.insert(resource));assert!(p.nodes.contains(&(resource-16)));
            if let Some(last)=history.insert(resource,*frame){let gap=frame.wrapping_sub(last);assert!(gap>=3);minimum_gap=minimum_gap.min(gap);}
            assert!(!pending.iter().any(|(_,address,_)|*address==resource));
            let value=(*frame as u64).wrapping_mul(65537)^item as u64;p.write_u64(resource+8,value);pending.push_back((*frame,resource,value));calls+=1;
        }
        check_lists(&p);
    }
    for(_,resource,value)in pending{assert_eq!(p.read_u64(resource+8),value);checksum=checksum.wrapping_mul(16777619)^value;}
    println!("{{\"case\":\"{}\",\"initial_pool\":{},\"frames\":{},\"allocations_requested\":{},\"distinct_resources\":{},\"new_resources\":{},\"minimum_reuse_gap\":{},\"logical_completion_checksum\":{},\"links_counts_canaries_preserved\":true,\"no_early_reuse\":true}}",name,count,frames.len(),calls,history.len(),p.allocations,if minimum_gap==u32::MAX{0}else{minimum_gap},checksum);
}
fn main(){
    assert_eq!(AUTHOR,"carunodev @stfu");
    run_case("steady",3000,80,(0..60).collect());
    run_case("skipped_frames",3000,80,vec![0,1,2,5,7,8,12,13,14,30]);
    run_case("exhaustion",8,5,(0..12).collect());
    run_case("frame_wrap",3000,80,vec![u32::MAX-2,u32::MAX-1,u32::MAX,0,1,2,3]);
}
