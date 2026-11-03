import re

__author__ = 'carunodev @stfu'
TARGET = 'f_005100_rage_clothmanager_instanceclothdrawabletask_rage_systaskparamete.rs'
CALLEE = 'f_005101_rage_clothmanager_instanceclothdrawable_rage_clothmanager_clothd'


def apply_task_upload_fixes(filename, text):
    if filename != TARGET:
        return text
    declaration = re.compile(
        r'pub unsafe extern "C" fn ' + re.escape(TARGET.removesuffix('.rs')) +
        r'\(ph: (?P<type>\*mut Phantom|&mut Phantom), mut l0: i64\) \{')
    matches = list(declaration.finditer(text))
    if len(matches) != 1 or text.count('fn ' + TARGET.removesuffix('.rs') + '(') != 1:
        raise ValueError('Unexpected InstanceClothDrawableTask declaration')
    match = matches[0]
    receiver = '(*ph)' if match['type'] == '*mut Phantom' else 'ph'
    locals_pattern = re.compile(
        r'(?P<newline>\r?\n)(?P<indent>[ \t]+)let mut l1: i64 = 0;\r?\n'
        r'(?P=indent)let mut l2: i64 = 0;\r?\n')
    locals_match = locals_pattern.match(text, match.end())
    if not locals_match:
        raise ValueError('Unexpected InstanceClothDrawableTask locals')
    newline, indent = locals_match['newline'], locals_match['indent']
    callbacks = newline.join([
        f'{indent}{receiver}.write_u64(({receiver}.g[28] as i64), 4095i64 as u64);',
        f'{indent}{receiver}.write_u64(({receiver}.g[29] as i64), 4096i64 as u64);',
        f'{indent}{receiver}.write_u64(12834520i64, 4097i64 as u64);',
    ]) + newline
    position = locals_match.end()
    existing = text.startswith(callbacks, position)
    markers = ('g[28]', 'g[29]', '12834520i64')
    expected_count = 1 if existing else 0
    if any(text.count(marker) != expected_count for marker in markers):
        raise ValueError('Unexpected InstanceClothDrawableTask upload registration')
    stack_position = position + (len(callbacks) if existing else 0)
    stack = re.compile(
        re.escape(indent + 'l1 = (' + receiver + '.sp.wrapping_add(') +
        r'(?:\(-64i64\)|-64i64)' + re.escape('));' + newline + indent + receiver + '.sp = l1;'))
    if not stack.match(text, stack_position):
        raise ValueError('Unexpected InstanceClothDrawableTask stack allocation')
    call = f'syms::{CALLEE}(ph, l2, l1);'
    if text.count(call) != 1:
        raise ValueError('Unexpected InstanceClothDrawableTask forwarding call')
    restore = re.compile(
        re.escape(call + newline + indent + receiver + '.sp = (l1.wrapping_sub(') +
        r'(?:\(-64i64\)|-64i64)' + re.escape('));'))
    if len(restore.findall(text)) != 1:
        raise ValueError('Unexpected InstanceClothDrawableTask stack restoration')
    return text if existing else text[:position] + callbacks + text[position:]
