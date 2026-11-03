import re

__author__ = 'carunodev @stfu'
WAIT = 'f_000311_wasm_null_d3d_hasgpuid_waitreadback'
FREE = 'f_091002_emscripten_builtin_free'
GROUPS = (
    ('texture2d', 144, (310, 312, 315, 316, 318, 319)),
    ('buffer', 120, (367, 368, 371, 372, 374, 375)),
    ('texture1d', 128, (379, 380, 382, 383, 385, 386)),
    ('texture3d', 136, (390, 391, 394, 395, 396, 397)),
)
TARGETS = {}
for resource, storage, indices in GROUPS:
    for position, index in enumerate(indices):
        suffix = ('wasm_null_d3d_nullresource_wasm_null_d3d_stub_id3d11' + resource +
                  ('_d3d11' if resource == 'buffer' else '_d3')) if position < 2 else 'non_virtual_thunk_to_wasm_null_d3d_nullresource_wasm_null_d3d_st'
        TARGETS[f'f_{index:06}_{suffix}.rs'] = (storage, (0, 0, 16, 16, 24, 24)[position], position % 2 == 1)


def apply_resource_readback_fixes(filename, text):
    if filename == WAIT + '.rs':
        return apply_readback_completion_guard(text)
    if filename not in TARGETS:
        return text
    storage, adjustment, _ = TARGETS[filename]
    name = filename.removesuffix('.rs')
    declaration = re.findall(r'pub unsafe extern "C" fn ' + re.escape(name) + r'\(ph: (\*mut Phantom|&mut Phantom), mut l0: i64\)', text)
    if len(declaration) != 1 or text.count('fn ' + name + '(') != 1:
        raise ValueError('Unexpected resource destructor declaration')
    receiver = '(*ph)' if declaration[0] == '*mut Phantom' else 'ph'
    newline = '\r\n' if '\r\n' in text else '\n'
    indent = '        '
    storage_local = 'l1' if adjustment == 0 else 'l2'
    offset = 24 - adjustment
    gpu = '(l0.wrapping_add(' + str(offset) + 'i64))' if offset else 'l0'
    pointer = 'l0 + ' + str(offset) if offset else 'l0'
    free = f'{indent}syms::{FREE}(ph, {storage_local});{newline}'
    vptr = f'{indent}{receiver}.write_u64({pointer}, 4092096i64 as u64);{newline}'
    wait = f'{indent}syms::{WAIT}(ph, {gpu});{newline}'
    storage_read = f'let t1: i64 = {receiver}.read_u64(l0 + {storage - adjustment}) as i64;'
    if text.count(storage_read) != 1 or text.count('syms::' + FREE + '(') != 2 or text.count('syms::' + WAIT + '(') != 1:
        raise ValueError('Unexpected resource storage or readback lifetime shape')
    original, fixed = free + vptr + wait, wait + free + vptr
    if text.count(fixed) == 1 and original not in text:
        return text
    if text.count(original) != 1 or fixed in text:
        raise ValueError('Unexpected resource destructor readback ordering')
    return text.replace(original, fixed, 1)


def apply_readback_completion_guard(text):
    declaration = re.findall(r'pub unsafe extern "C" fn ' + WAIT + r'\(ph: (\*mut Phantom|&mut Phantom), mut l0: i64\)', text)
    if len(declaration) != 1:
        raise ValueError('Unexpected readback wait declaration')
    receiver = '(*ph)' if declaration[0] == '*mut Phantom' else 'ph'
    newline = '\r\n' if '\r\n' in text else '\n'
    marker = '            syms::f_000138_wasm_wait_end(ph);' + newline
    guard = ('            if ' + receiver + '.a_load_u32(l0) == 1 {' + newline +
             '                rt::trap("GPU readback incomplete after wait limit");' + newline +
             '            }' + newline)
    if text.count(marker) != 1 or text.count('l0 = (l0.wrapping_add(68i64));') != 1 or text.count('l3 != 12000i32') != 1:
        raise ValueError('Unexpected readback wait completion shape')
    if marker + guard in text and text.count('GPU readback incomplete after wait limit') == 1:
        return text
    if 'GPU readback incomplete after wait limit' in text or receiver + '.a_load_u32(l0) == 1' in text:
        raise ValueError('Partial readback completion guard')
    return text.replace(marker, marker + guard, 1)
