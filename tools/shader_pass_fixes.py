import re

__author__ = 'carunodev @stfu'
TARGET = 'f_043670_csprite2d_begincustomlist_csprite2d_customshadertype_rage_grctex.rs'
FOW_PASSES = ((28, 'L24', 75, 15, 14), (36, 'L22', 80, 17, 16),
              (37, 'L21', 84, 18, 17), (38, 'L20', 85, 19, 18))


def apply_shader_pass_fixes(filename, text):
    if filename != TARGET:
        return text
    if text.count('fn ' + TARGET.removesuffix('.rs') + '(') != 1:
        raise ValueError('Unexpected BeginCustomList function declaration')
    edits = []
    for custom_type, label, temporary, old, new in FOW_PASSES:
        branch = re.compile(rf'^\s*{custom_type} => \{{ break \'{label}; \}}\s*$', re.MULTILINE)
        if len(branch.findall(text)) != 1:
            raise ValueError(f'Unexpected BeginCustomList dispatch for type {custom_type}')
        pattern = re.compile(
            rf'(?P<prefix>^[ \t]*let t{temporary}: i32 = (?:\(\*ph\)|ph)\.read_u32\(21395440i64\) as i32;\r?\n'
            rf'[ \t]*l54 = t{temporary};\r?\n[ \t]*l53 = )(?P<index>\d+)(?P<suffix>i32;)',
            re.MULTILINE)
        matches = list(pattern.finditer(text))
        if len(matches) != 1 or int(matches[0]['index']) not in (old, new):
            raise ValueError(f'Unexpected BeginCustomList pass for type {custom_type}')
        match = matches[0]
        edits.append((match.start('index'), match.end('index'), str(new)))
    for start, end, replacement in sorted(edits, reverse=True):
        text = text[:start] + replacement + text[end:]
    return text
