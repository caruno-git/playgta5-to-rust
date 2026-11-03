import ast
import hashlib
import json
from pathlib import Path
import unittest

from shader_pass_fixes import TARGET, FOW_PASSES, apply_shader_pass_fixes

__author__ = 'carunodev @stfu'
TOOLS = Path(__file__).resolve().parent
ROOT = TOOLS.parent


def fixture(receiver='(*ph)'):
    lines = ['pub unsafe extern "C" fn ' + TARGET.removesuffix('.rs') + '(ph: *mut Phantom) {', 'match l0 {']
    lines.extend(f'{kind} => {{ break \'{label}; }}' for kind, label, _, _, _ in FOW_PASSES)
    lines.append('}')
    for _, _, temporary, old, _ in FOW_PASSES:
        lines.extend([f'let t{temporary}: i32 = {receiver}.read_u32(21395440i64) as i32;',
                      f'l54 = t{temporary};', f'l53 = {old}i32;'])
    lines.extend(['l53 = 15i32;', 'l53 = 17i32;', 'l53 = 18i32;', 'l53 = 19i32;', '}'])
    return '\n'.join(lines)


class ShaderPassTests(unittest.TestCase):
    def test_pretransform_fixtures_match_provenance_hashes(self):
        directory = TOOLS / 'testdata'
        manifest = json.loads((directory / 'provenance.json').read_text(encoding='utf-8'))
        self.assertEqual(manifest['author'], 'carunodev @stfu')
        self.assertEqual(len(manifest['files']), 6)
        for record in manifest['files']:
            with self.subTest(fixture=record['fixture']):
                data = (directory / record['fixture']).read_bytes()
                self.assertEqual(len(data), record['fixture_bytes'])
                self.assertEqual(hashlib.sha256(data).hexdigest(), record['fixture_sha256'])
                if record['selection'] == 'complete pre-transform source file':
                    self.assertEqual(record['fixture_sha256'], record['source_sha256'])

    def test_only_four_exact_literals_change(self):
        original = fixture()
        fixed = apply_shader_pass_fixes(TARGET, original)
        before, after = original.splitlines(), fixed.splitlines()
        differences = [(old, new) for old, new in zip(before, after) if old != new]
        self.assertEqual(differences, [('l53 = 15i32;', 'l53 = 14i32;'), ('l53 = 17i32;', 'l53 = 16i32;'),
                                       ('l53 = 18i32;', 'l53 = 17i32;'), ('l53 = 19i32;', 'l53 = 18i32;')])
        self.assertEqual(after[-5:], before[-5:])

    def test_exact_idempotence(self):
        fixed = apply_shader_pass_fixes(TARGET, fixture())
        self.assertEqual(apply_shader_pass_fixes(TARGET, fixed), fixed)

    def test_unlinked_receiver_and_crlf_preserved(self):
        original = fixture('ph').replace('\n', '\r\n')
        fixed = apply_shader_pass_fixes(TARGET, original)
        self.assertEqual(fixed.count('\r\n'), original.count('\r\n'))
        self.assertIn('ph.read_u32', fixed)

    def test_other_function_unchanged(self):
        original = fixture()
        self.assertEqual(apply_shader_pass_fixes('f_044271_ui3ddrawmanager.rs', original), original)

    def test_rejects_missing_or_duplicate_context(self):
        for bad in [fixture().replace('let t75:', 'let t777:'), fixture() + '\n' + fixture()]:
            with self.assertRaises(ValueError):
                apply_shader_pass_fixes(TARGET, bad)

    def test_rejects_unexpected_existing_pass_or_dispatch(self):
        for bad in [fixture().replace('l53 = 15i32;', 'l53 = 99i32;', 1),
                    fixture().replace("28 => { break 'L24; }", "28 => { break 'L25; }")]:
            with self.assertRaises(ValueError):
                apply_shader_pass_fixes(TARGET, bad)

    def test_real_raw_source_changes_only_verified_passes(self):
        source = TOOLS / 'testdata' / TARGET
        original = source.read_text(encoding='utf-8')
        fixed = apply_shader_pass_fixes(TARGET, original)
        changed = [(old.strip(), new.strip()) for old, new in zip(original.splitlines(), fixed.splitlines()) if old != new]
        self.assertEqual(changed, [('l53 = 15i32;', 'l53 = 14i32;'), ('l53 = 17i32;', 'l53 = 16i32;'),
                                  ('l53 = 18i32;', 'l53 = 17i32;'), ('l53 = 19i32;', 'l53 = 18i32;')])

    def test_current_linked_source_already_has_fix(self):
        text = (ROOT / 'linked/generated/gen010/src' / TARGET).read_text(encoding='utf-8')
        self.assertEqual(apply_shader_pass_fixes(TARGET, text), text)

    def test_generator_calls_fix_and_preserves_conversion_barriers(self):
        tree = ast.parse((TOOLS / 'link_engine.py').read_text())
        barriers = next(node.value for node in tree.body if isinstance(node, ast.Assign) and
                        any(isinstance(target, ast.Name) and target.id == 'conversion_barriers' for target in node.targets))
        self.assertEqual(ast.literal_eval(barriers), {
            'f_021052_audnorthaudioengine_update.rs': ('s',),
            'f_035418_cminimap_renderthread_setupstateforsupertilerender_rage_vector2.rs': ('u',),
            'f_044343_centityselect_drawdebug.rs': ('u',),
        })
        calls = [node for node in ast.walk(tree) if isinstance(node, ast.Call) and
                 isinstance(node.func, ast.Name) and node.func.id == 'apply_shader_pass_fixes']
        self.assertEqual(len(calls), 1)
        self.assertEqual(ast.unparse(calls[0]), 'apply_shader_pass_fixes(source.name, text)')


if __name__ == '__main__':
    unittest.main()
