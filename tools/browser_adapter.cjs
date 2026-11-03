const fs = require('node:fs');
const path = require('node:path');
const author = 'carunodev @stfu';
const root = path.resolve(__dirname, '..');
const acorn = require('acorn');

function rustEngineDebug(ptr, len) {
  var limit = 65536, captured = null, available = null, truncated = false, decoderFallback = false, failure = null, message;
  var pointerText = typeof ptr === 'bigint' ? ptr >= -9223372036854775808n && ptr <= 18446744073709551615n ? String(ptr) : '<out-of-range bigint pointer>'
    : typeof ptr === 'number' ? String(ptr) : '<invalid pointer type>';
  var lengthText = typeof len === 'number' ? String(len) : '<invalid length type>';
  var describe = error => { try { return String(error && error.message || error).slice(0, 160); } catch (_) { return 'unavailable error details'; } };
  try {
    var buffer = typeof __rustMemory === 'undefined' || !__rustMemory ? null : __rustMemory.buffer;
    available = buffer && buffer.byteLength;
    var address = typeof ptr === 'bigint' ? ptr : Number.isSafeInteger(ptr) ? BigInt(ptr) : null;
    if (!Number.isSafeInteger(available) || available < 0 || address === null || address < 0n || address > BigInt(available)
      || !Number.isSafeInteger(len) || len < 0 || BigInt(len) > BigInt(available) - address) throw Error('Invalid Rust memory diagnostic range');
    var count = Math.min(len, limit);
    captured = new Uint8Array(count);
    captured.set(new Uint8Array(buffer, Number(address), count));
    truncated = count !== len;
    try { message = new TextDecoder('utf-8').decode(captured); }
    catch (error) {
      decoderFallback = true;
      message = '[Rust engine debug UTF-8 decoder unavailable: ' + describe(error) + ']\n';
      for (var index = 0; index < captured.length; index++) {
        var byte = captured[index];
        message += byte === 92 ? '\\\\' : byte === 9 || byte === 10 || byte === 13 || byte >= 32 && byte <= 126
          ? String.fromCharCode(byte) : '\\x' + byte.toString(16).padStart(2, '0');
      }
    }
    if (truncated) message += '\n[Rust engine debug truncated: captured ' + count + ' of ' + len + ' bytes]';
  } catch (error) {
    failure = describe(error);
    message = '[Rust engine debug unavailable: ' + failure + '; ptr=' + pointerText + '; len=' + lengthText + '; rustMemoryBytes=' + available + ']';
  }
  try {
    globalThis.__rustLastDebug = { text: message, requestedBytes: Number.isSafeInteger(len) && len >= 0 ? len : null,
      capturedBytes: captured ? captured.length : 0, maximumCapturedBytes: limit, truncated, decoderFallback, failure };
  } catch (_) {}
  try { console.error(message); } catch (_) {}
}

function engineDebugAssignment(ast) {
  const matches = [];
  function walk(node) {
    if (!node || typeof node !== 'object') return;
    if (node.type === 'AssignmentExpression' && node.left.type === 'MemberExpression' && node.left.object.name === 'wasmImports'
      && (node.left.computed ? node.left.property.value : node.left.property.name) === 'engine_debug') matches.push(node);
    for (const [key, value] of Object.entries(node)) {
      if (key === 'start' || key === 'end') continue;
      if (Array.isArray(value)) value.forEach(walk); else if (value && typeof value === 'object') walk(value);
    }
  }
  walk(ast);
  if (matches.length !== 1) throw Error('Expected exactly one Rust engine_debug assignment');
  return matches[0];
}

function patchEngineDebug(source) {
  if (typeof source !== 'string' || Buffer.byteLength(source) > 2 * 1024 * 1024) throw Error('Expected a bounded existing adapter source');
  const assignment = engineDebugAssignment(acorn.parse(source, { ecmaVersion: 'latest' }));
  const replacement = rustEngineDebug.toString();
  const patched = source.slice(0, assignment.right.start) + replacement + source.slice(assignment.right.end);
  acorn.parse(patched, { ecmaVersion: 'latest' });
  return { source: patched, previousRange: [assignment.right.start, assignment.right.end], replacementBytes: Buffer.byteLength(replacement) };
}

function adapterAstExceptDebug(source) {
  const ast = acorn.parse(source, { ecmaVersion: 'latest' });
  engineDebugAssignment(ast).right = { type: 'Identifier', name: '__RUST_ENGINE_DEBUG_ONLY__' };
  const normalize = value => {
    if (Array.isArray(value)) return value.map(normalize);
    if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value).filter(([key]) => !['start', 'end', 'raw'].includes(key)).map(([key, item]) => [key, normalize(item)]));
    return typeof value === 'bigint' ? { bigint: value.toString() } : value;
  };
  return JSON.stringify(normalize(ast));
}

function createPrelude(exportNames) {
  return `var __rustContext=0n,__rustRawExports=null,__rustFunctions=new Map(),__rustMemory=null;
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
}

function generateAdapter() {
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
    add('imports', first.end, first.end, `__rustMemory=__rustMemory||new WebAssembly.Memory({initial:${rustInitial}n,maximum:${rustMaximum}n,shared:true,address:"i64"});wasmImports.rust_memory=__rustMemory;wasmImports.engine_debug=${rustEngineDebug.toString()};wasmImports.engine_context=()=>{if(!__rustContext)throw Error("Rust context not initialized");return __rustContext;};for(const key of Object.keys(wasmImports)){if(key.startsWith("invoke_")){const invoke=wasmImports[key];wasmImports[key]=(...args)=>__rustCall(invoke,args);}}`);
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
const prelude = createPrelude(exportNames);
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
return { result, counts, exportNames, prelude };
}

if (require.main === module) {
  const args = process.argv.slice(2);
  if (args.length && (args.length !== 2 || args[0] !== '--output-dir')) throw Error('Usage: node browser_adapter.cjs [--output-dir NEW-directory]');
  const { result, counts, exportNames } = generateAdapter();
  const canonical = path.join(root, 'linked/browser');
  const destination = args.length ? path.resolve(args[1]) : canonical;
  let output = result, adapter = JSON.stringify({author,counts,exports:exportNames},null,2);
  if (args.length) {
    if (fs.existsSync(destination) || destination === canonical) throw Error('Diagnostic output directory must be new and separate from canonical browser files');
    const baseline = fs.readFileSync(path.join(canonical, 'game.js'), 'utf8');
    const patched = patchEngineDebug(baseline);
    if (adapterAstExceptDebug(result) !== adapterAstExceptDebug(baseline) || adapterAstExceptDebug(patched.source) !== adapterAstExceptDebug(baseline))
      throw Error('Adapter AST outside engine_debug differs from canonical');
    output = patched.source;
    const existingAdapter = fs.readFileSync(path.join(canonical, 'adapter.json'), 'utf8');
    const current = JSON.parse(existingAdapter);
    if (current.author !== author || JSON.stringify(current.exports) !== JSON.stringify(exportNames)
      || Object.keys(current.counts).length !== Object.keys(counts).length || Object.entries(counts).some(([key, value]) => current.counts[key] !== value))
      throw Error('Canonical adapter metadata differs from generated ABI');
    adapter = existingAdapter;
  }
  fs.mkdirSync(destination, {recursive:!args.length});
  fs.writeFileSync(path.join(destination, 'game.js'), output, args.length ? {flag:'wx'} : undefined);
  fs.writeFileSync(path.join(destination, 'adapter.json'), adapter, args.length ? {flag:'wx'} : undefined);
  console.log(counts);
}

module.exports = { author, rustEngineDebug, engineDebugAssignment, patchEngineDebug, adapterAstExceptDebug, createPrelude, generateAdapter };
