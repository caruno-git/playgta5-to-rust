const fs = require('node:fs');
const path = require('node:path');
const author = 'carunodev @stfu';
const root = path.resolve(__dirname, '..');
const acorn = require('acorn');
const input = fs.readFileSync(process.env.PLAYGTA5_JS || path.join(root, 'inputs/game.js'), 'utf8');
const ast = acorn.parse(input, {ecmaVersion: 'latest'});
const config=JSON.parse(fs.readFileSync(path.join(root,'linked/browser/memory-config.json'),'utf8'));
if(!Array.isArray(config.rust_memory_pages)||!config.rust_memory_pages[0])throw Error('Missing Rust memory dimensions');
const [rustInitial,rustMaximum]=config.rust_memory_pages;
const patches = [], counts = {};
function add(name, start, end, text) {
  counts[name] = (counts[name] || 0) + 1;
  patches.push({start, end, text});
}
function visit(node) {
  if (!node || typeof node !== 'object') return;
  if (node.type === 'FunctionDeclaration' && node.id.name === 'getWasmImports') {
    const first = node.body.body[0];
    if (first.expression?.callee?.name !== 'assignWasmImports') throw Error('Unexpected imports initializer');
    add('imports', first.end, first.end, `__rustMemory=__rustMemory||new WebAssembly.Memory({initial:${rustInitial}n,maximum:${rustMaximum}n,shared:true,address:"i64"});wasmImports.rust_memory=__rustMemory;wasmImports.engine_debug=(ptr,len)=>console.error(new TextDecoder().decode(new Uint8Array(__rustMemory.buffer,Number(ptr),len)));wasmImports.engine_context=()=>{if(!__rustContext)throw Error("Rust context not initialized");return __rustContext;};for(const key of Object.keys(wasmImports)){if(key.startsWith("invoke_")){const invoke=wasmImports[key];wasmImports[key]=(...args)=>__rustCall(invoke,args);}}`);
  }
  if (node.type === 'AssignmentExpression' && node.left.name === 'wasmExports' &&
      node.right.type === 'MemberExpression' && node.right.object.name === 'instance' && node.right.property.name === 'exports')
    add('exports', node.right.start, node.right.end, '__rustWrapExports(instance.exports)');
  if (node.type === 'VariableDeclarator' && node.id.name === 'getWasmTableEntry') {
    if (node.init.body.type !== 'BlockStatement' || node.init.params[0].name !== 'funcPtr') throw Error('Unexpected table accessor');
    add('table', node.init.body.start + 1, node.init.body.start + 1, 'if(__rustRawExports)return __rustTableEntry(funcPtr);');
  }
  if (node.type === 'CallExpression' && node.callee.type === 'MemberExpression' && node.callee.property.name === 'postMessage') {
    const message = node.arguments[0];
    if (message?.type === 'ObjectExpression') {
      const fields = new Map(message.properties.filter(p => p.type === 'Property').map(p => [p.key.name || p.key.value, p.value]));
      if (fields.get('cmd')?.value === 1 && fields.has('wasmMemory') && fields.has('wasmModule'))
        add('worker_send', message.start + 1, message.start + 1, '__rustMemory,__rustStackTop:__rustRawExports.engine_allocate_stack(0),');
    }
  }
  if (node.type === 'IfStatement' && node.test.type === 'BinaryExpression' && node.test.left.name === 'cmd' &&
      node.test.right.value === 1 && node.consequent.type === 'BlockStatement' && input.slice(node.consequent.start, node.consequent.end).includes('msgData.wasmMemory')) {
    add('worker_receive', node.consequent.start + 1, node.consequent.start + 1, '__rustMemory=msgData.__rustMemory;globalThis.__rustStackTop=msgData.__rustStackTop;');
    const tail=node.consequent.body.slice(-3);
    if(tail.map(n=>n.expression?.callee?.name).join(',')!=='createWasm,run,startWorker')throw Error('Unexpected worker boot sequence');
    add('worker_lazy',tail[0].start,tail[2].end,'__rustDeferWorker(messageQueue,createWasm,run,handleMessage);');
  }
  for (const [key, value] of Object.entries(node)) {
    if (key === 'start' || key === 'end') continue;
    if (Array.isArray(value)) value.forEach(visit);
    else if (value && typeof value === 'object') visit(value);
  }
}
visit(ast);
for (const name of ['imports','exports','table','worker_send','worker_receive','worker_lazy'])
  if (counts[name] !== 1) throw Error(`${name}: expected one patch, got ${counts[name]}`);
const exportNames = JSON.parse(fs.readFileSync(path.join(root, 'linked/browser-exports.json'), 'utf8')).map(e => e[0]);
const prelude = `var __rustContext=0n,__rustRawExports=null,__rustFunctions=new Map(),__rustMemory=null;
function __rustDeferWorker(queue,instantiate,run,handle){
  var starting=false;
  self.onmessage=event=>{
    queue.push(event);
    if(!starting&&event.data.cmd===2){
      starting=true;
      Promise.resolve().then(instantiate).then(()=>{
        run();self.onmessage=handle;
        for(const message of queue)handle(message);
        queue.length=0;
      }).catch(error=>{console.error('Rust worker initialization failed',error);throw error;});
    }
  };
  postMessage({cmd:3});
}
function __rustCall(invoke,args){
  var stack=__rustRawExports.__stack_pointer.value;
  try{return invoke(...args);}finally{__rustRawExports.__stack_pointer.value=stack;}
}
function __rustWrapExports(raw){
  __rustRawExports=raw;
  if(ENVIRONMENT_IS_PTHREAD){
    if(typeof globalThis.__rustStackTop!=='bigint')throw Error('Missing Rust worker stack');
    raw.__stack_pointer.value=globalThis.__rustStackTop;
  }else{
    raw.__stack_pointer.value=raw.engine_allocate_stack(16777216);
  }
  __rustContext=raw.engine_new_context();
  console.log('[rust] context '+__rustContext+' stack '+raw.__stack_pointer.value);
  if(!__rustContext)throw Error('Rust constructor returned a null context');
  var wrapped={__indirect_function_table:raw.__indirect_function_table};
  for(const name of ${JSON.stringify(exportNames)}){
    if(typeof raw['engine_'+name]!=='function')throw Error('Missing Rust export '+name);
    const invoke=raw['engine_'+name];
    wrapped[name]=(...args)=>__rustCall(invoke,args);
  }
  return wrapped;
}
function __rustTableEntry(index){
  var key=BigInt(index),fn=__rustFunctions.get(key);
  if(!fn){
    var type=__rustRawExports.engine_function_type(key),dispatch=__rustRawExports['engine_dispatch_'+type];
    if(typeof dispatch!=='function')throw Error('Invalid Rust table slot '+index+' type '+type);
    fn=(...args)=>__rustCall(dispatch,[key,...args]);__rustFunctions.set(key,fn);
  }
  return fn;
}
`;
let result = input;
for (const patch of patches.sort((a,b) => b.start - a.start)) result = result.slice(0,patch.start) + patch.text + result.slice(patch.end);
result = prelude + result;
acorn.parse(result, {ecmaVersion: 'latest'});
const vm=require('node:vm');
const unit={console:{log(){}}};
vm.createContext(unit);
vm.runInContext(prelude,unit);
vm.runInContext(`__rustRawExports={__stack_pointer:{value:1024n}};
try{__rustCall(()=>{__rustRawExports.__stack_pointer.value=512n;throw 42;},[]);}catch(error){if(error!==42)throw error;}
if(__rustRawExports.__stack_pointer.value!==1024n)throw Error('Exception leaked the Rust stack');
if(__rustCall((a,b)=>a+b,[3,4])!==7)throw Error('Return value changed');`,unit);
unit.self={};unit.postMessage=message=>{if(message.cmd!==3)throw Error('Invalid worker readiness message');};
vm.runInContext(`var instantiated=0,ran=0,handled=[];
__rustDeferWorker([],()=>{instantiated++;},()=>{ran++;},event=>handled.push(event.data.cmd));
if(instantiated!==0)throw Error('Idle worker instantiated eagerly');
self.onmessage({data:{cmd:4}});self.onmessage({data:{cmd:2}});self.onmessage({data:{cmd:4}});`,unit);
setImmediate(()=>{
  if(unit.instantiated!==1||unit.ran!==1||unit.handled.join(',')!=='4,2,4')throw Error('Lazy worker queue lost messages');
});
const destination = path.join(root, 'linked/browser');
fs.mkdirSync(destination, {recursive:true});
fs.writeFileSync(path.join(destination, 'game.js'), result);
fs.writeFileSync(path.join(destination, 'adapter.json'), JSON.stringify({author,counts,exports:exportNames},null,2));
console.log(counts);
