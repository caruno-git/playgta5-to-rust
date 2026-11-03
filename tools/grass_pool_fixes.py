import hashlib
import re

__author__ = 'carunodev @stfu'
TARGET = 'f_046589_ebstatic_preallocatedresourcepool_ebstatic_preallocatedappendbuf.rs'
BASELINE_SHA256 = '9663b434211e0d8f36d0a3fbe6463dc2bec39e6470d766e13a48f0ca33af0bb8'
ORIGINAL = 'let t55: i64 = ph.read_u64(23333280i64) as i64;'
FIXED = 'let t55: i64 = ph.read_u64(if (r38 as u32) >= 3 { 23333272i64 } else { 23333280i64 }) as i64;'


def apply_grass_pool_fixes(filename, text):
    if filename != TARGET:
        return text
    declaration = re.findall(r'pub unsafe extern "C" fn ' + re.escape(TARGET.removesuffix('.rs')) + r'\(ph: (\*mut Phantom|&mut Phantom)\) -> i64', text)
    if len(declaration) != 1:
        raise ValueError('Unexpected grass append-buffer allocator signature')
    normalized = text.replace('\r\n', '\n').replace('ph: *mut Phantom', 'ph: &mut Phantom').replace('(*ph).', 'ph.')
    if normalized.count(FIXED) == 1 and ORIGINAL not in normalized:
        baseline = normalized.replace(FIXED, ORIGINAL, 1)
        already_fixed = True
    elif normalized.count(ORIGINAL) == 1 and FIXED not in normalized:
        baseline = normalized
        already_fixed = False
    else:
        raise ValueError('Unexpected grass append-buffer selection shape')
    if hashlib.sha256(baseline.encode('utf-8')).hexdigest() != BASELINE_SHA256:
        raise ValueError('Grass pool lifetime source changed outside reviewed selection')
    if already_fixed:
        return text
    receiver = '(*ph)' if declaration[0] == '*mut Phantom' else 'ph'
    return text.replace(ORIGINAL.replace('ph.', receiver + '.'), FIXED.replace('ph.', receiver + '.'), 1)
