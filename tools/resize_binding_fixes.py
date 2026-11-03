import re

__author__ = 'carunodev @stfu'
TARGET = 'f_009678_rage_grcdevice_prepareforshutdown.rs'
CLEAR_NAMES = (
    'f_010440_ragedirect3ddevicecontext11_csclearconstantbuffers',
    'f_010441_ragedirect3ddevicecontext11_dsclearconstantbuffers',
    'f_010442_ragedirect3ddevicecontext11_gsclearconstantbuffers',
    'f_010443_ragedirect3ddevicecontext11_hsclearconstantbuffers',
    'f_010444_ragedirect3ddevicecontext11_psclearconstantbuffers',
    'f_010445_ragedirect3ddevicecontext11_vsclearconstantbuffers',
)
RESET_PATTERN = re.compile(
    r'(?P<prefix>l2 = \(\((?P<receiver>\(\*ph\)|ph)\.g\[1\] as i64\)\.wrapping_add\(1160i64\)\);\r?\n'
    r'(?P<indent>[ \t]*)let t1: i64 = (?P=receiver)\.read_u64\(l2\) as i64;\r?\n'
    r'(?P=indent)l0 = t1;\r?\n)'
    r'(?P<calls>(?:(?P=indent)syms::f_01044[0-5]_ragedirect3ddevicecontext11_[cdghpv]sclearconstantbuffers\(ph, l0\);\r?\n)*)'
    r'(?P<suffix>(?P=indent)let t2: i64 = (?P=receiver)\.read_u64\(l0\) as i64;\r?\n'
    r'(?P=indent)let t3: i64 = (?P=receiver)\.read_u64\(t2 \+ 64\) as i64;)')


def apply_resize_binding_fixes(filename, text):
    if filename != TARGET:
        return text
    name = TARGET.removesuffix('.rs')
    declaration = re.compile(r'pub unsafe extern "C" fn ' + name + r'\(ph: (\*mut Phantom|&mut Phantom)\) \{')
    declarations = list(declaration.finditer(text))
    if len(declarations) != 1 or text.count('fn ' + name + '(') != 1:
        raise ValueError('Unexpected PrepareForShutdown declaration')
    matches = list(RESET_PATTERN.finditer(text))
    if len(matches) != 1:
        raise ValueError('Unexpected PrepareForShutdown TLS wrapper or first unbind block')
    match = matches[0]
    receiver = '(*ph)' if declarations[0][1] == '*mut Phantom' else 'ph'
    if match['receiver'] != receiver:
        raise ValueError('Unexpected PrepareForShutdown receiver')
    newline = '\r\n' if '\r\n' in match[0] else '\n'
    wanted = ''.join(f'{match["indent"]}syms::{name}(ph, l0);{newline}' for name in CLEAR_NAMES)
    if match['calls'] not in ('', wanted):
        raise ValueError('Unexpected PrepareForShutdown constant-buffer clear block')
    expected = 1 if match['calls'] else 0
    if any(text.count('syms::' + name + '(') != expected for name in CLEAR_NAMES):
        raise ValueError('Unexpected PrepareForShutdown constant-buffer clear calls')
    return text[:match.start('calls')] + wanted + text[match.end('calls'):]


def apply_resize_binding_symbols(crate, filename, text):
    if crate != 'gen002' or filename != 'syms.rs':
        return text
    name = TARGET.removesuffix('.rs')
    declaration = re.compile(r'extern "C" \{ pub fn ' + name + r'\(ph: (?P<receiver>\*mut Phantom|&mut Phantom)\); \}')
    matches = list(declaration.finditer(text))
    if len(matches) != 1 or text.count('pub fn ' + name + '(') != 1:
        raise ValueError('Unexpected gen002 PrepareForShutdown ABI declaration')
    receiver = matches[0]['receiver']
    missing = []
    for name in CLEAR_NAMES:
        wanted = f'extern "C" {{ pub fn {name}(ph: {receiver}, a0: i64); }}'
        count = text.count('pub fn ' + name + '(')
        if count:
            if count != 1 or text.count(wanted) != 1:
                raise ValueError('Unexpected gen002 constant-buffer clear ABI declaration')
        else:
            missing.append(wanted)
    if not missing:
        return text
    marker = 'use wasm_rt::Phantom;'
    if text.count(marker) != 1:
        raise ValueError('Unexpected gen002 Phantom import')
    newline = '\r\n' if '\r\n' in text else '\n'
    return text.replace(marker, marker + newline + newline.join(missing), 1)
