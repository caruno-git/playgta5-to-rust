import ast
import hashlib
from pathlib import Path
import unittest

from minimap_mode_fixes import (
    MODE_EXPRESSION, MODE_NAME, RESET_PATTERN, TARGET,
    apply_minimap_mode_fixes, apply_minimap_mode_symbols,
)

__author__ = 'carunodev @stfu'
TOOLS = Path(__file__).resolve().parent
ROOT = TOOLS.parent
RAW = TOOLS / 'testdata' / TARGET
LINKED = ROOT / 'linked/generated/gen008/src' / TARGET


class MinimapModeTests(unittest.TestCase):
    def test_real_source_changes_only_one_non_reinit_default_store(self):
        raw_bytes = RAW.read_bytes()
        original = raw_bytes.decode('utf-8')
        fixed = apply_minimap_mode_fixes(TARGET, original)
        changes = [(old.strip(), new.strip()) for old, new in zip(original.splitlines(), fixed.splitlines()) if old != new]
        receiver = RESET_PATTERN.search(original)['receiver']
        self.assertEqual(changes, [(f'{receiver}.write_u8(19520605i64, 0i32 as u8);',
                                    f'{receiver}.write_u8(19520605i64, {MODE_EXPRESSION});')])
        self.assertEqual(hashlib.sha256(RAW.read_bytes()).digest(), hashlib.sha256(raw_bytes).digest())
        self.assertEqual(fixed.count('syms::' + MODE_NAME + '(ph)'), 1)
        self.assertNotIn('syms::f_000568_wasm_start_mode(ph)', fixed)

    def test_idempotent_and_current_linked_source_matches_policy(self):
        text = LINKED.read_text(encoding='utf-8')
        self.assertEqual(apply_minimap_mode_fixes(TARGET, text), text)
        raw = RAW.read_text(encoding='utf-8')
        fixed = apply_minimap_mode_fixes(TARGET, raw)
        self.assertEqual(apply_minimap_mode_fixes(TARGET, fixed), fixed)

    def test_phase_eight_only_and_reinit_preservation_are_source_verified(self):
        text = LINKED.read_text(encoding='utf-8')
        self.assertEqual(text.count("7 => { break 'L3; }"), 1)
        match = RESET_PATTERN.search(text)
        self.assertIsNotNone(match)
        self.assertIn('if ((t209 == 0) as i32) != 0', match[0])
        reinit = (ROOT / 'linked/generated/gen008/src/f_035296_cminimap_reinit.rs').read_text()
        enabled = reinit.index('write_u8(19520607i64, 1i32 as u8)')
        init = reinit.index('f_035298_cminimap_init_unsigned_int(ph, 8i32)')
        disabled = reinit.index('write_u8(19520607i64, 0i32 as u8)')
        self.assertLess(enabled, init)
        self.assertLess(init, disabled)

    def test_other_functions_and_other_crate_symbols_are_unchanged(self):
        text = RAW.read_text()
        self.assertEqual(apply_minimap_mode_fixes('f_035424_render.rs', text), text)
        symbols = (ROOT / 'linked/generated/gen008/src/syms.rs').read_text()
        self.assertEqual(apply_minimap_mode_symbols('gen007', 'syms.rs', symbols), symbols)
        self.assertEqual(apply_minimap_mode_symbols('gen008', 'lib.rs', symbols), symbols)

    def test_crlf_and_raw_receiver_preserved(self):
        text = RAW.read_text().replace('\n', '\r\n')
        fixed = apply_minimap_mode_fixes(TARGET, text)
        self.assertEqual(fixed.count('\r\n'), text.count('\r\n'))
        self.assertEqual(RESET_PATTERN.search(fixed)['receiver'], RESET_PATTERN.search(text)['receiver'])

    def test_unexpected_scope_value_or_duplicate_block_fail_closed(self):
        text = RAW.read_text()
        variants = [text.replace('write_u8(19520605i64, 0i32 as u8)', 'write_u8(19520605i64, 7i32 as u8)'),
                    text.replace('t209 == 0', 't209 == 1', 1),
                    text.replace("7 => { break 'L3; }", "7 => { break 'L4; }"), text + text]
        for broken in variants:
            with self.assertRaises(ValueError):
                apply_minimap_mode_fixes(TARGET, broken)

    def test_only_gen008_adds_exact_existing_mode_now_abi_once(self):
        raw = (TOOLS / 'testdata/gen008_syms.rs').read_text()
        fixed = apply_minimap_mode_symbols('gen008', 'syms.rs', raw)
        self.assertEqual(fixed.count('pub fn ' + MODE_NAME + '('), 1)
        self.assertIn(f'pub fn {MODE_NAME}(ph: &mut Phantom) -> i32;', fixed)
        self.assertEqual(apply_minimap_mode_symbols('gen008', 'syms.rs', fixed), fixed)
        linked = (ROOT / 'linked/generated/gen008/src/syms.rs').read_text()
        self.assertEqual(apply_minimap_mode_symbols('gen008', 'syms.rs', linked), linked)
        self.assertIn(f'pub fn {MODE_NAME}(ph: *mut Phantom) -> i32;', linked)
        with self.assertRaises(ValueError):
            apply_minimap_mode_symbols('gen008', 'syms.rs', linked.replace('-> i32; }', '-> i64; }'))

    def test_link_hook_preserves_prior_fixes_and_retranslation_delegation(self):
        source = (TOOLS / 'link_engine.py').read_text()
        tree = ast.parse(source)
        names = ['apply_shader_pass_fixes', 'apply_task_upload_fixes', 'apply_minimap_mode_fixes', 'apply_minimap_mode_symbols']
        for name in names:
            calls = [node for node in ast.walk(tree) if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == name]
            self.assertEqual(len(calls), 1, name)
        self.assertIn('f_035418_cminimap_renderthread_setupstateforsupertilerender_rage_vector2.rs', source)
        retrans = ast.parse((TOOLS / 'retranslate_linked.py').read_text())
        self.assertTrue(any(isinstance(node, ast.Constant) and isinstance(node.value, str) and node.value == 'tools/link_engine.py' for node in ast.walk(retrans)))


if __name__ == '__main__':
    unittest.main()
