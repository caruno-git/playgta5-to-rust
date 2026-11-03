import ast
import hashlib
import json
from pathlib import Path
import unittest

from resource_readback_fixes import FREE, TARGETS, WAIT, apply_resource_readback_fixes

__author__ = 'carunodev @stfu'
ROOT = Path(__file__).resolve().parent.parent
RAW = ROOT / 'tools/testdata/readback'
LINKED = ROOT / 'linked/generated/gen000/src'


class ResourceReadbackFixTests(unittest.TestCase):
    def test_exact_pretransform_fixtures_match_relative_provenance(self):
        manifest = json.loads((RAW / 'provenance.json').read_text(encoding='utf-8'))
        self.assertEqual(manifest['author'], __author__)
        self.assertEqual({item['fixture'] for item in manifest['files']}, set(TARGETS) | {WAIT + '.rs'})
        for item in manifest['files']:
            with self.subTest(fixture=item['fixture']):
                self.assertEqual(Path(item['fixture']).name, item['fixture'])
                self.assertEqual(item['source'], 'linked/raw-generated/gen000/src/' + item['fixture'])
                content = (RAW / item['fixture']).read_bytes()
                self.assertEqual(len(content), item['fixture_bytes'])
                self.assertEqual(hashlib.sha256(content).hexdigest(), item['fixture_sha256'])
                self.assertEqual(item['fixture_bytes'], item['source_bytes'])
                self.assertEqual(item['fixture_sha256'], item['source_sha256'])

    def test_all_actual_destructor_variants_only_move_existing_wait_before_storage_free(self):
        self.assertEqual(len(TARGETS), 24)
        for filename in TARGETS:
            with self.subTest(filename=filename):
                original = (RAW / filename).read_text(encoding='utf-8')
                fixed = apply_resource_readback_fixes(filename, original)
                old_lines, new_lines = original.splitlines(), fixed.splitlines()
                wait = next(line for line in old_lines if 'syms::' + WAIT + '(' in line)
                self.assertEqual([line for line in old_lines if line != wait], [line for line in new_lines if line != wait])
                self.assertLess(original.index('syms::' + FREE), original.index('syms::' + WAIT))
                self.assertLess(fixed.index('syms::' + WAIT), fixed.index('syms::' + FREE))
                self.assertEqual(apply_resource_readback_fixes(filename, fixed), fixed)
                linked = (LINKED / filename).read_text(encoding='utf-8')
                self.assertEqual(apply_resource_readback_fixes(filename, linked), linked)
                self.assertEqual(fixed.replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).'), linked)

    def test_crlf_and_other_destructors_are_preserved(self):
        for filename in TARGETS:
            source = (RAW / filename).read_text(encoding='utf-8').replace('\n', '\r\n')
            fixed = apply_resource_readback_fixes(filename, source)
            self.assertEqual(fixed.count('\r\n'), source.count('\r\n'))
            self.assertNotIn('\n', fixed.replace('\r\n', ''))
            self.assertEqual(apply_resource_readback_fixes('f_000320_wasm_null_d3d_hasgpuid_dtorhasgpuid.rs', source), source)

    def test_drift_duplicate_wait_and_partial_transforms_fail_closed(self):
        filename = next(iter(TARGETS))
        source = (RAW / filename).read_text(encoding='utf-8')
        wait = next(line + '\n' for line in source.splitlines() if 'syms::' + WAIT + '(' in line)
        for broken in [source + source, source.replace('ph: &mut Phantom', 'ph: &Phantom'),
                       source.replace('read_u64(l0 + 144)', 'read_u64(l0 + 136)'),
                       source.replace('wrapping_add(24i64)', 'wrapping_add(16i64)'),
                       source.replace(wait, wait + wait), source.replace(wait, ''),
                       source.replace('4092096i64', '4092097i64')]:
            with self.assertRaises(ValueError):
                apply_resource_readback_fixes(filename, broken)

    def test_generator_has_one_regeneration_hook(self):
        tree = ast.parse((ROOT / 'tools/link_engine.py').read_text(encoding='utf-8'))
        calls = [node for node in ast.walk(tree) if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == 'apply_resource_readback_fixes']
        self.assertEqual(len(calls), 1)
        self.assertEqual(ast.unparse(calls[0]), 'apply_resource_readback_fixes(source.name, text)')

    def test_wait_guard_checks_pending_ack_after_wait_before_returning_to_callers(self):
        filename = WAIT + '.rs'
        source = (RAW / filename).read_text(encoding='utf-8')
        fixed = apply_resource_readback_fixes(filename, source)
        self.assertLess(fixed.index('f_000138_wasm_wait_end'), fixed.index('GPU readback incomplete'))
        self.assertLess(fixed.index('GPU readback incomplete'), fixed.index('let t5: f64'))
        self.assertEqual(apply_resource_readback_fixes(filename, fixed), fixed)
        self.assertEqual(fixed.replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).'), (LINKED / filename).read_text(encoding='utf-8'))
        for broken in [source + source, source.replace('l3 != 12000i32', 'l3 != 12001i32'), source.replace('wrapping_add(68i64)', 'wrapping_add(64i64)')]:
            with self.assertRaises(ValueError):
                apply_resource_readback_fixes(filename, broken)


if __name__ == '__main__':
    unittest.main()
