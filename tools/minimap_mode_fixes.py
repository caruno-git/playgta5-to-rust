import re

__author__ = 'carunodev @stfu'
TARGET = 'f_035298_cminimap_init_unsigned_int.rs'
MODE_NAME = 'f_000566_wasm_start_mode_now'
MODE_EXPRESSION = f'((syms::{MODE_NAME}(ph) == 2i32) as u8)'
RESET_PATTERN = re.compile(
    r'(?P<prefix>let t209: i32 = (?P<receiver>\(\*ph\)|ph)\.read_u8\(19520607i64\) as i32;\r?\n'
    r'[ \t]*if \(\(t209 == 0\) as i32\) != 0 \{\r?\n'
    r'[ \t]*(?P=receiver)\.write_u8\(19520604i64, 0i32 as u8\);\r?\n'
    r'[ \t]*(?P=receiver)\.write_u8\(6974156i64, 1i32 as u8\);\r?\n'
    r'[ \t]*(?P=receiver)\.write_u8\(19520605i64, )(?P<value>[^;\r\n]+)(?P<suffix>\);\r?\n[ \t]*\})')


def apply_minimap_mode_fixes(filename, text):
    if filename != TARGET:
        return text
    if text.count('fn ' + TARGET.removesuffix('.rs') + '(') != 1:
        raise ValueError('Unexpected CMiniMap Init declaration')
    if text.count("7 => { break 'L3; }") != 1 or text.count("0 => { break 'L4; }") != 1:
        raise ValueError('Unexpected CMiniMap Init phase-8 dispatch')
    matches = list(RESET_PATTERN.finditer(text))
    if len(matches) != 1 or text.count('write_u8(19520605i64,') != 1:
        raise ValueError('Unexpected CMiniMap Init non-ReInit default block')
    match = matches[0]
    if match['value'] not in ('0i32 as u8', MODE_EXPRESSION):
        raise ValueError('Unexpected CMiniMap Init HideFoW default expression')
    expected_calls = 1 if match['value'] == MODE_EXPRESSION else 0
    if text.count('syms::' + MODE_NAME + '(ph)') != expected_calls:
        raise ValueError('Unexpected CMiniMap Init mode-query calls')
    return text[:match.start('value')] + MODE_EXPRESSION + text[match.end('value'):]


def apply_minimap_mode_symbols(crate, filename, text):
    if crate != 'gen008' or filename != 'syms.rs':
        return text
    declaration = re.compile(
        r'extern "C" \{ pub fn f_035298_cminimap_init_unsigned_int\(ph: '
        r'(?P<receiver>\*mut Phantom|&mut Phantom), a0: i32\); \}')
    matches = list(declaration.finditer(text))
    if len(matches) != 1:
        raise ValueError('Unexpected gen008 Init callee declaration')
    receiver = matches[0]['receiver']
    wanted = f'extern "C" {{ pub fn {MODE_NAME}(ph: {receiver}) -> i32; }}'
    count = text.count('pub fn ' + MODE_NAME + '(')
    if count:
        if count != 1 or text.count(wanted) != 1:
            raise ValueError('Unexpected gen008 mode-query ABI declaration')
        return text
    newline = '\r\n' if '\r\n' in text else '\n'
    marker = 'use wasm_rt::Phantom;'
    if text.count(marker) != 1:
        raise ValueError('Unexpected gen008 Phantom import')
    return text.replace(marker, marker + newline + wanted, 1)
