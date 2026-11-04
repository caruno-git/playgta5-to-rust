'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const crypto=require('node:crypto');
const author='carunodev @stfu';
const sourcePath=path.resolve(__dirname,'../browser/io_worker.js');
const sourceBytes=fs.readFileSync(sourcePath),source=sourceBytes.toString('utf8');
const expectedSha256='9f5d4262c699ef8490dee16aca9b4cfa3e2d26e1d5c17cfa58f1d8e9c40103d6';
const tick=()=>new Promise(resolve=>setImmediate(resolve));

function harness({automaticDemand=19}={}){
  const requests=[],chunks=new Map(),notifications=[];
  const memory=new WebAssembly.Memory({initial:32,maximum:64,shared:true});
  let released=false;
  const trackedAtomics=new Proxy(Atomics,{get(target,key){
    if(key==='notify')return (array,index,count)=>{notifications.push({index,state:array[index]});return Atomics.notify(array,index,count);};
    return Reflect.get(target,key);
  }});
  const context=vm.createContext({console,performance,WebAssembly,Uint8Array,Int32Array,DataView,ArrayBuffer,SharedArrayBuffer,Atomics:trackedAtomics,TextEncoder,
    self:{postMessage(){}},BroadcastChannel:class{postMessage(){}},setTimeout(){return 1;},clearTimeout(){},setInterval(){},
    fetch:async(url,init)=>{
      const id=Number(/fixture(\d+)\.rpf/.exec(String(url))[1]),match=/^bytes=(\d+)-(\d+)$/.exec(init.headers.Range);
      const record={id,start:Number(match[1]),end:Number(match[2]),priority:init.priority};
      let release;const gate=new Promise(resolve=>release=resolve);record.release=release;requests.push(record);
      if(released||id===automaticDemand&&init.priority==='high'&&record.start===0)release();
      let delivered=false;
      return {ok:true,status:206,body:{getReader:()=>({
        async read(){
          if(delivered)return {done:true};
          await gate;delivered=true;
          if(record.bodyError)throw Error(record.bodyError);
          return {done:false,value:new Uint8Array(record.end-record.start+1).fill(id+1)};
        },async cancel(){}
      })}};
    }
  });
  vm.runInContext(source,context,{filename:sourcePath});
  context.fixtureMemory=memory;
  context.fixtureStore={write(data,{at}){chunks.set(at,data.slice());return data.length;},
    read(data,{at}){
      for(const [offset,chunk]of chunks)if(offset<=at&&offset+chunk.length>=at+data.length){data.set(chunk.subarray(at-offset,at-offset+data.length));return data.length;}
      throw Error('Missing fixture extent');
    },flush(){}};
  vm.runInContext(`
    mem=fixtureMemory;HEAP32=new Int32Array(mem.buffer);HEAPU8=new Uint8Array(mem.buffer);table=0;
    files=Array.from({length:20},(_,id)=>({path:'fixture'+id+'.rpf',size:1048576}));sh=fixtureStore;bc={postMessage(){}};
    globalThis.api={
      hints(count,low=false){for(let id=0;id<count;id++)(low?lowQueue:hintQueue).push([id,0,1048576]);pumpHints();},
      demand(id,length=4096,destination=8192){views();HEAP32.set([1,id,0,0,destination,0,length,-999],16);return serve(0);},
      isSpeculative(id,block=0){return lowPriorityRuns.has(inflight.get(id*1048576+block));},
      bytes(destination=8192){views();return HEAPU8.slice(destination,destination+4096);},
      state(){return {slot:HEAP32[16],result:HEAP32[23],active:networkActive,maxActive:netStat.maxActive,lowActive:networkLowActive,queued:{...networkQueued},
        promotions:networkPromotions,inflight:inflight.size,hintActive,lowActiveJobs:lowActive};}
    };
  `,context);
  return {api:context.api,requests,memory,notifications,
    async close(){
      released=true;for(const request of requests)request.release();
      for(let n=0;n<200;n++){await tick();const state=context.api.state();if(!state.inflight&&!state.hintActive&&!state.lowActiveJobs&&!state.active)return;}
      throw Error('Fixture did not settle');
    }};
}

test('published worker has the exact tested source identity',()=>{
  assert.equal(crypto.createHash('sha256').update(sourceBytes).digest('hex'),expectedSha256);
  assert.equal(sourceBytes.length,45155);assert.match(source,/PATCH_AUTHOR = "carunodev @stfu"/);
  console.log(JSON.stringify({author,workerSha256:expectedSha256,bytes:sourceBytes.length,networkUsed:false}));
});

test('normal hint fanout is limited to 88 active operations and leaves room for a high-priority demand',async()=>{
  const h=harness();let read;
  try{
    h.api.hints(16);await tick();assert.equal(h.requests.length,88);
    assert.ok(h.requests.every(value=>value.priority==='low'&&value.end-value.start+1===131072));
    assert.equal(h.api.state().lowActive,0);assert.equal(h.api.isSpeculative(0),false);
    read=h.api.demand(19);await tick();await tick();
    const demand=h.requests.filter(value=>value.id===19);assert.equal(demand.length,1);assert.equal(demand[0].priority,'high');
    assert.equal(h.api.state().slot,2);assert.equal(h.api.state().result,4096);assert.ok(h.api.bytes().every(value=>value===20));
    assert.ok(h.api.state().maxActive<=96);assert.equal(h.notifications.length,1);
  }finally{await h.close();await read;}
});

test('a demanded queued normal hint is promoted to high HTTP priority without a duplicate transfer',async()=>{
  const h=harness({automaticDemand:15});let read;
  try{
    h.api.hints(16);await tick();assert.equal(h.requests.some(value=>value.id===15),false);
    read=h.api.demand(15);await tick();await tick();
    const promoted=h.requests.filter(value=>value.id===15);assert.equal(promoted.length,1);
    assert.equal(promoted[0].priority,'high');assert.equal(promoted[0].start,0);assert.equal(promoted[0].end,131071);
    assert.equal(h.api.state().promotions,1);assert.equal(h.api.state().slot,2);assert.ok(h.api.bytes().every(value=>value===16));
  }finally{await h.close();await read;}
});

test('a demand joining an active normal hint completes with its own slice without duplicating it',async()=>{
  const h=harness({automaticDemand:0});let read;
  try{
    h.api.hints(1);await tick();assert.equal(h.requests.length,8);assert.equal(h.requests[0].priority,'low');
    assert.equal(h.api.isSpeculative(0),false);read=h.api.demand(0);await tick();
    assert.equal(h.requests.length,8);assert.equal(h.api.state().slot,1);assert.equal(h.api.state().promotions,0);
    h.requests.find(value=>value.id===0&&value.start===0).release();await read;
    assert.equal(h.api.state().slot,2);assert.equal(h.api.state().result,4096);assert.ok(h.api.bytes().every(value=>value===1));
    assert.ok(h.api.state().hintActive>0);assert.equal(h.notifications.length,1);
    await h.api.demand(0);assert.equal(h.requests.length,8);assert.equal(h.api.state().result,4096);
  }finally{await h.close();await read;}
});

test('speculative hints retain large slices and their separate urgent fallback',async()=>{
  const h=harness({automaticDemand:0});let read;
  try{
    h.api.hints(1,true);await tick();assert.equal(h.requests.length,1);assert.equal(h.requests[0].priority,'low');
    assert.equal(h.requests[0].end,1048575);assert.equal(h.api.isSpeculative(0),true);
    read=h.api.demand(0);await tick();await tick();assert.equal(h.requests.length,2);
    assert.equal(h.requests[1].priority,'high');assert.equal(h.requests[1].start,0);assert.equal(h.requests[1].end,4095);
    assert.equal(h.api.state().slot,2);assert.equal(h.api.state().result,4096);assert.ok(h.api.bytes().every(value=>value===1));
  }finally{await h.close();await read;}
});

test('failure of the required normal hint reports one error without using incomplete bytes',async()=>{
  const h=harness();let read;
  try{
    h.api.hints(1);await tick();read=h.api.demand(0);await tick();
    h.requests[0].bodyError='Fixture body failed';h.requests[0].release();await read;
    assert.equal(h.api.state().result,-1);assert.equal(h.api.state().slot,2);assert.equal(h.notifications.length,1);
    assert.equal(h.api.bytes().some(Boolean),false);assert.ok(h.api.state().inflight>0);assert.ok(h.api.state().active>0);
  }finally{await h.close();await read;}
});

test('an in-flight read refreshes its shared-memory view after growth',async()=>{
  const h=harness();let read;
  try{
    h.api.hints(1);await tick();const destination=2*1048576+4096;read=h.api.demand(0,4096,destination);
    h.memory.grow(32);h.requests[0].release();await read;
    assert.equal(h.api.state().result,4096);assert.equal(h.api.state().slot,2);assert.equal(h.notifications.length,1);
    assert.ok(h.api.bytes(destination).every(value=>value===1));
  }finally{await h.close();await read;}
});
