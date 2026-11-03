'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const vm = require('node:vm');
const author = 'carunodev @stfu';
const canonicalPath = path.resolve(__dirname, '../linked/browser/game.js');
const adapter = require('./browser_adapter.cjs');
const digest = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const preparedInputs = [canonicalPath, process.env.PLAYGTA5_JS || path.resolve(__dirname, '../inputs/game.js'),
  path.resolve(__dirname, '../linked/browser/memory-config.json'), path.resolve(__dirname, '../linked/browser-exports.json')];
const missingPreparedInputs = preparedInputs.filter(file => !fs.existsSync(file));

function environment(buffer, options = {}) {
  const logs = [], inputs = [];
  class BrowserDecoder {
    constructor(label) { assert.equal(label === undefined || label === 'utf-8', true); }
    decode(bytes) {
      if (Object.prototype.toString.call(bytes.buffer) === '[object SharedArrayBuffer]') throw TypeError('The provided ArrayBufferView value must not be shared.');
      inputs.push(bytes);
      options.onDecode?.(bytes);
      if (options.decoderFailure) throw options.decoderFailure;
      return new TextDecoder('utf-8').decode(bytes);
    }
  }
  const context = { __rustMemory: buffer === null ? null : { buffer }, TextDecoder: options.Decoder || BrowserDecoder,
    console: { log() {}, error(message) { if (options.loggerFailure) throw options.loggerFailure; logs.push(message); } } };
  vm.createContext(context);
  vm.runInContext('var fixedDebug = (' + adapter.rustEngineDebug.toString() + ');', context);
  return { context, logs, inputs, invoke: (ptr, len) => context.fixedDebug(ptr, len) };
}

test('browser-like shared-view decoder reproduces original callback failure and the fixed callback exposes the exact UTF-8 panic', () => {
  const message = 'Rust engine: panic: unsupported C++ dynamic_cast \u043E\u0448\u0438\u0431\u043A\u0430 \u00E9 \u4E2D', encoded = new TextEncoder().encode(message);
  const shared = new SharedArrayBuffer(encoded.length + 64); new Uint8Array(shared).set(encoded, 32);
  const state = environment(shared);
  vm.runInContext('var oldDebug = (ptr,len)=>console.error(new TextDecoder().decode(new Uint8Array(__rustMemory.buffer,Number(ptr),len)));', state.context);
  assert.throws(() => state.context.oldDebug(32n, encoded.length), /must not be shared/);
  assert.doesNotThrow(() => state.invoke(32n, encoded.length));
  assert.equal(state.logs[0], message);
  assert.equal(Object.prototype.toString.call(state.inputs[0].buffer), '[object ArrayBuffer]');
  assert.equal(state.context.__rustLastDebug.text, message);
  assert.equal(state.context.__rustLastDebug.capturedBytes, encoded.length);
  assert.equal(state.context.__rustLastDebug.truncated, false);
});

test('private copy is complete before decoder execution and does not retain the source shared view', () => {
  const message = 'Rust engine: original panic bytes', encoded = new TextEncoder().encode(message), source = new SharedArrayBuffer(encoded.length);
  new Uint8Array(source).set(encoded);
  const state = environment(source, { onDecode() { new Uint8Array(source).fill(88); } });
  state.invoke(0n, encoded.length);
  assert.equal(state.logs[0], message);
  assert.notEqual(state.inputs[0].buffer, source);
});

test('nonshared Rust memory, exact zero-length boundaries and fresh memory buffers are supported', () => {
  const old = new TextEncoder().encode('first panic'), next = new TextEncoder().encode('second panic');
  const state = environment(old.buffer);
  state.invoke(0n, old.length); assert.equal(state.logs[0], 'first panic');
  state.context.__rustMemory.buffer = next.buffer;
  state.invoke(0n, next.length); assert.equal(state.logs[1], 'second panic');
  state.invoke(BigInt(next.length), 0); assert.equal(state.logs[2], '');
});

test('oversized valid panic messages are bounded and visibly marked rather than silently truncated', () => {
  const bytes = new Uint8Array(new SharedArrayBuffer(65536 + 37)); bytes.fill(65);
  const state = environment(bytes.buffer); state.invoke(0n, bytes.length);
  const record = state.context.__rustLastDebug;
  assert.equal(record.capturedBytes, 65536); assert.equal(record.requestedBytes, 65573); assert.equal(record.truncated, true);
  assert.equal(state.inputs[0].length, 65536);
  assert.match(state.logs[0], /captured 65536 of 65573 bytes/);
  assert.equal(state.logs[0].startsWith('A'.repeat(65536)), true);
});

test('malformed diagnostic ranges never read unrelated heap bytes or replace the original panic with a secondary range error', () => {
  const source = new SharedArrayBuffer(64); new Uint8Array(source).set(new TextEncoder().encode('do-not-dump-unrelated-memory'));
  for (const [pointer, length] of [[-1n, 1], [65n, 0], [63n, 2], [0n, -1], [0n, 1.5], [1.5, 1], [1n << 100000n, 1], [0n, 64n]]) {
    const state = environment(source);
    assert.doesNotThrow(() => state.invoke(pointer, length));
    assert.equal(state.inputs.length, 0); assert.equal(state.context.__rustLastDebug.capturedBytes, 0);
    assert.match(state.logs[0], /Rust engine debug unavailable/);
    assert.doesNotMatch(state.logs[0], /do-not-dump-unrelated-memory/);
    assert.ok(state.logs[0].length < 512);
  }
  const missing = environment(null); assert.doesNotThrow(() => missing.invoke(0n, 1)); assert.match(missing.logs[0], /unavailable/);
});

test('decoder failure retains readable panic ASCII and bounded escaped byte evidence', () => {
  const message = new TextEncoder().encode('Rust engine: panic C:\\source\n\u0435'), state = environment(message.buffer, { decoderFailure: Error('decoder deliberately unavailable') });
  state.invoke(0n, message.length);
  assert.match(state.logs[0], /Rust engine: panic C:\\\\source/);
  assert.match(state.logs[0], /\\xd0\\xb5/);
  assert.equal(state.context.__rustLastDebug.decoderFallback, true);
  assert.equal(state.context.__rustLastDebug.failure, null);
});

test('missing decoder or failing console does not throw from the panic diagnostic import and retains one bounded last diagnostic', () => {
  const message = new TextEncoder().encode('Rust engine: original panic');
  const state = environment(message.buffer, { loggerFailure: Error('console unavailable') });
  state.context.TextDecoder = undefined;
  assert.doesNotThrow(() => state.invoke(0n, message.length));
  assert.match(state.context.__rustLastDebug.text, /Rust engine: original panic/);
  assert.equal(state.context.__rustLastDebug.decoderFallback, true);
  assert.equal(state.logs.length, 0);
});

test('the underlying original trap still propagates unchanged through __rustCall after diagnostics and Rust stack restoration', () => {
  const encoded = new TextEncoder().encode('Rust engine: underlying dynamic_cast panic'), state = environment(encoded.buffer);
  vm.runInContext(adapter.createPrelude([]), state.context);
  state.context.__rustMemory = { buffer: encoded.buffer };
  state.context.__rustRawExports = { __stack_pointer: { value: 1024n } };
  const sentinel = Error('original Rust unreachable trap'); state.context.originalTrap = sentinel;
  state.context.debugLength = encoded.length;
  const invoke = vm.runInContext('() => { __rustRawExports.__stack_pointer.value=512n; fixedDebug(0n,debugLength); throw originalTrap; }', state.context);
  assert.throws(() => state.context.__rustCall(invoke, []), error => error === sentinel);
  assert.equal(state.context.__rustRawExports.__stack_pointer.value, 1024n);
  assert.equal(state.logs[0], 'Rust engine: underlying dynamic_cast panic');
});

test('patching a bounded adapter changes only the panic callback and is idempotent', () => {
  const original = 'var wasmImports={};wasmImports.engine_debug=(ptr,len)=>console.error(ptr,len);var unrelated=42;';
  const patched = adapter.patchEngineDebug(original), replacement = adapter.rustEngineDebug.toString();
  assert.equal(adapter.adapterAstExceptDebug(patched.source), adapter.adapterAstExceptDebug(original));
  assert.equal(patched.source.slice(0, patched.previousRange[0]), original.slice(0, patched.previousRange[0]));
  assert.equal(patched.source.slice(patched.previousRange[0] + replacement.length), original.slice(patched.previousRange[1]));
  assert.equal(adapter.patchEngineDebug(patched.source).source, patched.source);
  assert.equal(adapter.author, author);
});

test('all generated/copy-candidate AST outside engine_debug and every canonical source byte outside that callback are unchanged', {
  skip: missingPreparedInputs.length ? 'Requires prepared inputs/game.js (or PLAYGTA5_JS) and linked/browser game.js plus memory-config.json; see UPDATE-2026-11-04.md' : false
}, () => {
  const canonicalBefore = fs.readFileSync(canonicalPath), canonical = canonicalBefore.toString('utf8');
  const generated = adapter.generateAdapter();
  const patched = adapter.patchEngineDebug(canonical), range = patched.previousRange, replacement = adapter.rustEngineDebug.toString();
  assert.equal(adapter.adapterAstExceptDebug(generated.result), adapter.adapterAstExceptDebug(canonical));
  assert.equal(adapter.adapterAstExceptDebug(patched.source), adapter.adapterAstExceptDebug(canonical));
  assert.equal(patched.source.slice(0, range[0]), canonical.slice(0, range[0]));
  assert.equal(patched.source.slice(range[0] + replacement.length), canonical.slice(range[1]));
  assert.equal(adapter.author, author);
  assert.ok(Object.values(generated.counts).every(value => value === 1));
  assert.equal(digest(fs.readFileSync(canonicalPath)), digest(canonicalBefore));
  assert.equal(adapter.patchEngineDebug(patched.source).source, patched.source);
});

test('invalid or ambiguous adapter sources reject before producing a replacement', () => {
  assert.throws(() => adapter.patchEngineDebug('var unrelated = 1;'), /exactly one/);
  assert.throws(() => adapter.patchEngineDebug('wasmImports.engine_debug=()=>{};wasmImports.engine_debug=()=>{};'), /exactly one/);
  assert.throws(() => adapter.patchEngineDebug('x'.repeat(2 * 1024 * 1024 + 1)), /bounded/);
});
