'use strict';
const fs = require('node:fs');
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const test = require('node:test');
const author = 'carunodev @stfu';
const file = process.argv[2] || process.env.PLAYGTA5_IM_FXC;
test('four FOW passes match the independently supplied exact im.fxc contract', {
  skip: file ? false : 'Requires exact im.fxc argument or PLAYGTA5_IM_FXC; see UPDATE-2026-11-04.md',
}, () => {
const data = fs.readFileSync(file);
assert.equal(crypto.createHash('sha256').update(data).digest('hex'), '5fb9c81b98186b81186159302b027443077ebc5d311644b062e532c2d7b60700');
assert.equal(data.toString('ascii', 0, 4), 'rgxe');
assert.equal(data.readUInt32LE(4), 89);
assert.equal(data[8], 0);
let cursor = 9;
const bound = (offset, size) => {assert.ok(Number.isInteger(offset) && Number.isInteger(size) && offset >= 0 && size >= 0 && offset + size <= data.length); return offset;};
const byte = () => data[bound(cursor++, 1)];
const string = () => {
  const size = byte(), start = cursor;
  bound(start, size); cursor += size;
  assert.ok(size > 0 && data[cursor - 1] === 0);
  return data.toString('ascii', start, cursor - 1);
};
const fnv64 = bytes => {
  let hash = 0xcbf29ce484222325n;
  for (const value of bytes) hash = BigInt.asUintN(64, (hash ^ BigInt(value)) * 0x100000001b3n);
  return hash.toString(16).padStart(16, '0');
};
function programs() {
  const count = byte(); assert.ok(count > 0 && count <= 128);
  return Array.from({length: count}, (_, index) => {
    const name = string(), parameters = Array.from({length: byte()}, string), bindings = byte();
    for (let i = 0; i < bindings; i++) {string(); byte(); byte();}
    const size = data.readUInt32LE(bound(cursor, 4)); cursor += 4;
    const start = cursor; bound(cursor, size); cursor += size;
    const blob = data.subarray(start, start + size);
    if (size) {assert.equal(blob.toString('ascii', 0, 4), 'DXBC'); assert.equal(blob.readUInt32LE(24), size); byte(); byte();}
    return {index, name, parameters, blob, hash: size ? fnv64(blob) : null};
  });
}
function textures(blob) {
  const count = blob.readUInt32LE(28), output = [];
  assert.ok(count <= 32 && 32 + count * 4 <= blob.length);
  for (let i = 0; i < count; i++) {
    const offset = blob.readUInt32LE(32 + i * 4);
    assert.ok(offset + 8 <= blob.length);
    const length = blob.readUInt32LE(offset + 4); assert.ok(offset + 8 + length <= blob.length);
    if (blob.toString('ascii', offset, offset + 4) !== 'RDEF') continue;
    const chunk = blob.subarray(offset + 8, offset + 8 + length), resources = chunk.readUInt32LE(8), start = chunk.readUInt32LE(12);
    assert.ok(resources <= 128 && start + resources * 32 <= chunk.length);
    for (let resource = 0; resource < resources; resource++) {
      const at = start + resource * 32;
      if (chunk.readUInt32LE(at + 4) !== 2) continue;
      const name = chunk.readUInt32LE(at), end = chunk.indexOf(0, name);
      assert.ok(name < chunk.length && end >= name && end - name < 256);
      output.push({name: chunk.toString('ascii', name, end), slot: chunk.readUInt32LE(at + 20), count: chunk.readUInt32LE(at + 24)});
    }
  }
  return output;
}
const vertex = programs(), pixel = programs();
assert.equal(vertex.length, 34); assert.equal(pixel.length, 78);
const name = Buffer.from('gtadrawblit\0'), offset = data.indexOf(name);
assert.ok(offset > 0); assert.equal(data.indexOf(name, offset + 1), -1); assert.equal(data[offset - 1], name.length);
cursor = offset + name.length;
const passCount = byte(); assert.equal(passCount, 24);
const passes = Array.from({length: passCount}, (_, index) => {
  const stages = Array.from({length: 6}, byte), states = byte(); bound(cursor, states * 8); cursor += states * 8;
  assert.ok(vertex[stages[0]] && pixel[stages[1]]);
  return {index, vertex: vertex[stages[0]], pixel: pixel[stages[1]]};
});
assert.equal(string(), 'gtadrawblit_do');
const expected = [
  [28, 14, 'VS_GTABlit', '028921879df627f5', 'PS_BlitToAlpha', 'bc10d4d035fb7863', [{name:'DiffuseSampler',slot:2,count:1}]],
  [36, 16, 'VS_Blit', '4e1d5d361e0ed0bc', 'PS_FOWCount', 'ccc76e271d50c641', [{name:'RenderMapPointSampler',slot:6,count:1}]],
  [37, 17, 'VS_Blit', '4e1d5d361e0ed0bc', 'PS_FOWAverage', '3bc052f6178cafad', [{name:'RenderMapPointSampler',slot:6,count:1}]],
  [38, 18, 'VS_GTABlit', '028921879df627f5', 'PS_FOWFill', '161bc912b03bda5f', []],
];
for (const [type, pass, vertexName, vertexHash, pixelName, pixelHash, resources] of expected) {
  const actual = passes[pass];
  assert.equal(actual.vertex.name, vertexName); assert.equal(actual.vertex.hash, vertexHash);
  assert.equal(actual.pixel.name, pixelName); assert.equal(actual.pixel.hash, pixelHash);
  assert.deepEqual(textures(actual.pixel.blob), resources);
  console.log(`PASS custom type ${type} -> gtadrawblit pass ${pass}: ${vertexName} / ${pixelName}; exact resource slots`);
}
assert.equal(passes[15].pixel.name, 'PS_BlitDepthToAlpha');
assert.ok(textures(passes[15].pixel.blob).some(texture => texture.name === 'DepthMapPointSampler' && texture.slot === 5));
console.log('PASS replaced FOW pass 15 is independently distinguished from the required FOW shader; ' + author);
});
