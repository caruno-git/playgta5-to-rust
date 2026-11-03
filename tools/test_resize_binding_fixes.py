import ast
import hashlib
from pathlib import Path
import unittest

from resize_binding_fixes import (
    CLEAR_NAMES, RESET_PATTERN, TARGET,
    apply_resize_binding_fixes, apply_resize_binding_symbols,
)

__author__ = 'carunodev @stfu'
TOOLS = Path(__file__).resolve().parent
ROOT = TOOLS.parent
RAW = TOOLS / 'testdata' / TARGET
LINKED = ROOT / 'linked/generated/gen002/src' / TARGET
SYMS = ROOT / 'linked/generated/gen002/src/syms.rs'


class ResizeBindingTests(unittest.TestCase):
    def test_real_source_inserts_only_six_cache_clears_before_first_unbind(self):
        raw_bytes = RAW.read_bytes()
        original = raw_bytes.decode('utf-8')
        fixed = apply_resize_binding_fixes(TARGET, original)
        match = RESET_PATTERN.search(fixed)
        self.assertEqual(fixed[:match.start('calls')] + fixed[match.end('calls'):], original)
        calls = [line.strip() for line in match['calls'].splitlines()]
        self.assertEqual(calls, [f'syms::{name}(ph, l0);' for name in CLEAR_NAMES])
        self.assertLess(fixed.index('l0 = t1;'), match.start('calls'))
        self.assertLess(match.end('calls'), fixed.index('t5(ph as *mut Phantom'))
        self.assertEqual(hashlib.sha256(RAW.read_bytes()).digest(), hashlib.sha256(raw_bytes).digest())
        for name in CLEAR_NAMES:
            self.assertEqual(fixed.count('syms::' + name + '(ph, l0);'), 1)

    def test_idempotence_and_current_linked_source(self):
        raw = RAW.read_text(encoding='utf-8')
        fixed = apply_resize_binding_fixes(TARGET, raw)
        self.assertEqual(apply_resize_binding_fixes(TARGET, fixed), fixed)
        linked = LINKED.read_text(encoding='utf-8')
        self.assertEqual(apply_resize_binding_fixes(TARGET, linked), linked)
        match = RESET_PATTERN.search(linked)
        original = linked[:match.start('calls')] + linked[match.end('calls'):]
        self.assertEqual(apply_resize_binding_fixes(TARGET, original), linked)
        self.assertEqual(len(match['calls'].splitlines()), 6)

    def test_borrowed_and_pointer_receivers_and_crlf_are_preserved(self):
        sources = [RAW.read_text(encoding='utf-8'), LINKED.read_text(encoding='utf-8')]
        for source in sources:
            for newline in ('\n', '\r\n'):
                original = source.replace('\n', newline)
                fixed = apply_resize_binding_fixes(TARGET, original)
                self.assertEqual(RESET_PATTERN.search(fixed)['receiver'], RESET_PATTERN.search(original)['receiver'])
                self.assertEqual(fixed.replace(newline, '').count('\n'), 0)
                self.assertEqual(apply_resize_binding_fixes(TARGET, fixed), fixed)

    def test_other_functions_crates_and_files_are_unchanged(self):
        source = RAW.read_text(encoding='utf-8')
        self.assertEqual(apply_resize_binding_fixes('f_010016_rage_grcdevice_forcedevicereset.rs', source), source)
        symbols = SYMS.read_text(encoding='utf-8')
        self.assertEqual(apply_resize_binding_symbols('gen001', 'syms.rs', symbols), symbols)
        self.assertEqual(apply_resize_binding_symbols('gen002', 'lib.rs', symbols), symbols)

    def test_unexpected_shutdown_source_drift_is_rejected(self):
        source = RAW.read_text(encoding='utf-8')
        variants = [
            source.replace('wrapping_add(1160i64)', 'wrapping_add(1168i64)', 1),
            source.replace('l0 = t1;', 'l0 = t2;', 1),
            source.replace('read_u64(t2 + 64)', 'read_u64(t2 + 72)', 1),
            source.replace('ph: &mut Phantom', 'ph: &Phantom', 1),
            source.replace('let t1: i64 = ph.read_u64(l2)', 'let t1: i64 = (*ph).read_u64(l2)', 1),
            source + source,
        ]
        for broken in variants:
            with self.subTest(variant=variants.index(broken)), self.assertRaises(ValueError):
                apply_resize_binding_fixes(TARGET, broken)

    def test_partial_duplicate_reordered_or_wrong_clear_calls_are_rejected(self):
        fixed = apply_resize_binding_fixes(TARGET, RAW.read_text(encoding='utf-8'))
        call = f'        syms::{CLEAR_NAMES[0]}(ph, l0);\n'
        second = f'        syms::{CLEAR_NAMES[1]}(ph, l0);\n'
        variants = [
            fixed.replace(call, '', 1),
            fixed.replace(call, call + call, 1),
            fixed.replace(call + second, second + call, 1),
            fixed.replace(call, call.replace('(ph, l0)', '(ph, l2)'), 1),
            fixed + call,
        ]
        for broken in variants:
            with self.subTest(variant=variants.index(broken)), self.assertRaises(ValueError):
                apply_resize_binding_fixes(TARGET, broken)

    def test_symbols_add_only_six_existing_clear_abis_once(self):
        raw = (TOOLS / 'testdata/gen002_syms.rs').read_text(encoding='utf-8')
        fixed = apply_resize_binding_symbols('gen002', 'syms.rs', raw)
        declarations = [f'extern "C" {{ pub fn {name}(ph: &mut Phantom, a0: i64); }}' for name in CLEAR_NAMES]
        self.assertEqual(fixed.replace('\n'.join(declarations) + '\n', '', 1), raw)
        self.assertEqual(apply_resize_binding_symbols('gen002', 'syms.rs', fixed), fixed)
        linked = SYMS.read_text(encoding='utf-8')
        self.assertEqual(apply_resize_binding_symbols('gen002', 'syms.rs', linked), linked)
        stripped = linked
        for name in CLEAR_NAMES:
            declaration = f'extern "C" {{ pub fn {name}(ph: *mut Phantom, a0: i64); }}\n'
            stripped = stripped.replace(declaration, '', 1)
        expected = raw.replace('ph: &mut Phantom', 'ph: *mut Phantom')
        selected = ''.join(line for line in stripped.splitlines(keepends=True)
                           if line.startswith('use wasm_rt::Phantom;') or 'pub fn ' + TARGET.removesuffix('.rs') + '(' in line)
        self.assertEqual(selected, expected)
        for name in CLEAR_NAMES:
            self.assertEqual(fixed.count('pub fn ' + name + '('), 1)
            self.assertEqual(linked.count('pub fn ' + name + '('), 1)
            self.assertIn(f'pub fn {name}(ph: *mut Phantom, a0: i64);', linked)
            wrapper = (LINKED.parent / (name + '.rs')).read_text(encoding='utf-8')
            self.assertIn(f'pub unsafe extern "C" fn {name}(ph: *mut Phantom, mut l0: i64)', wrapper)

    def test_symbol_crlf_is_preserved(self):
        source = (TOOLS / 'testdata/gen002_syms.rs').read_text(encoding='utf-8').replace('\n', '\r\n')
        fixed = apply_resize_binding_symbols('gen002', 'syms.rs', source)
        self.assertEqual(fixed.count('\r\n'), source.count('\r\n') + 6)
        self.assertNotIn('\n', fixed.replace('\r\n', ''))
        self.assertEqual(apply_resize_binding_symbols('gen002', 'syms.rs', fixed), fixed)

    def test_unexpected_symbol_abi_drift_is_rejected(self):
        symbols = SYMS.read_text(encoding='utf-8')
        declaration = f'extern "C" {{ pub fn {CLEAR_NAMES[0]}(ph: *mut Phantom, a0: i64); }}'
        variants = [
            symbols.replace(declaration, declaration.replace('a0: i64', 'a0: i32'), 1),
            symbols.replace(declaration, declaration.replace('*mut Phantom', '&mut Phantom'), 1),
            symbols.replace(declaration, declaration.replace('extern "C"', 'extern "system"'), 1),
            symbols + '\n' + declaration,
            symbols.replace('f_009678_rage_grcdevice_prepareforshutdown(ph: *mut Phantom)',
                            'f_009678_rage_grcdevice_prepareforshutdown(ph: *mut Phantom, a0: i64)', 1),
        ]
        for broken in variants:
            with self.subTest(variant=variants.index(broken)), self.assertRaises(ValueError):
                apply_resize_binding_symbols('gen002', 'syms.rs', broken)
        raw = (TOOLS / 'testdata/gen002_syms.rs').read_text(encoding='utf-8')
        with self.assertRaises(ValueError):
            apply_resize_binding_symbols('gen002', 'syms.rs', raw.replace('use wasm_rt::Phantom;', ''))

    def test_link_hook_preserves_previous_fixes_barriers_and_retranslation(self):
        source = (TOOLS / 'link_engine.py').read_text(encoding='utf-8')
        tree = ast.parse(source)
        names = ['apply_shader_pass_fixes', 'apply_task_upload_fixes', 'apply_minimap_mode_fixes',
                 'apply_minimap_mode_symbols', 'apply_resize_binding_fixes', 'apply_resize_binding_symbols']
        lines = []
        for name in names:
            calls = [node for node in ast.walk(tree) if isinstance(node, ast.Call)
                     and isinstance(node.func, ast.Name) and node.func.id == name]
            self.assertEqual(len(calls), 1, name)
            lines.append(calls[0].lineno)
        self.assertEqual(lines, sorted(lines))
        self.assertIn('if source.name in conversion_barriers:', source)
        self.assertIn('f_035418_cminimap_renderthread_setupstateforsupertilerender_rage_vector2.rs', source)
        self.assertIn('"    #[inline(never)]"', source)
        retrans = ast.parse((TOOLS / 'retranslate_linked.py').read_text(encoding='utf-8'))
        self.assertTrue(any(isinstance(node, ast.Constant) and node.value == 'tools/link_engine.py'
                            for node in ast.walk(retrans)))


if __name__ == '__main__':
    unittest.main()
