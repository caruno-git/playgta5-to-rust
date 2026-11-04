'use strict';
const fs=require('node:fs');
const author='carunodev @stfu';
const opcodes={'v128.load8_splat':7,'v128.load16_splat':8,'v128.load32_splat':9,'v128.load64_splat':10,'v128.load32_zero':92,'v128.load64_zero':93};
function uleb(value){const bytes=[];do{let byte=value&127;value=Math.floor(value/128);if(value)byte|=128;bytes.push(byte);}while(value);return bytes;}
const section=(kind,bytes)=>[kind,...uleb(bytes.length),...bytes];
const name=value=>[...uleb(value.length),...Buffer.from(value)];
function moduleBytes(op,offset){
  if(!(op in opcodes)||!Number.isSafeInteger(offset)||offset<0||offset>16)throw Error('Invalid memory opcode or offset');
  const code=[0,0x41,0,0x20,0,0xfd,...uleb(opcodes[op]),0,...uleb(offset),0xfd,11,0,0,0x0b];
  return new Uint8Array([0,97,115,109,1,0,0,0,
    ...section(1,[1,0x60,1,0x7f,0]),...section(3,[1,0]),...section(5,[1,0,1]),
    ...section(7,[2,...name('memory'),2,0,...name('run'),0,0]),...section(10,[1,...uleb(code.length),...code])]);
}
function run(cases){
  return cases.map(({op,offset,address})=>{
    const instance=new WebAssembly.Instance(new WebAssembly.Module(moduleBytes(op,offset)));
    const memory=new Uint8Array(instance.exports.memory.buffer);for(let i=0;i<memory.length;i++)memory[i]=(i*37+129)&255;
    try{instance.exports.run(address);return {op,offset,address,trap:false,hex:Buffer.from(memory.subarray(0,16)).toString('hex')};}
    catch(error){if(!(error instanceof WebAssembly.RuntimeError))throw error;return {op,offset,address,trap:true};}
  });
}
if(require.main===module)process.stdout.write(JSON.stringify({author,results:run(JSON.parse(fs.readFileSync(0,'utf8')))}));
module.exports={author,moduleBytes,run};
