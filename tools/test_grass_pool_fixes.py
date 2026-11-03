import ast
import hashlib
import json
from pathlib import Path
import unittest

from grass_pool_fixes import BASELINE_SHA256, FIXED, ORIGINAL, TARGET, apply_grass_pool_fixes

__author__ = 'carunodev @stfu'
ROOT = Path(__file__).resolve().parent.parent
FIXTURES = ROOT / 'tools/testdata/grass_pool'
RAW = FIXTURES / TARGET
LINKED = ROOT / 'linked/generated/gen011/src' / TARGET


class GrassPoolFixTests(unittest.TestCase):
    def test_pretransform_fixture_and_harness_have_exact_relative_provenance(self):
        manifest = json.loads((FIXTURES / 'provenance.json').read_text(encoding='utf-8'))
        self.assertEqual(manifest['author'], __author__)
        self.assertEqual({entry['fixture'] for entry in manifest['files']}, {TARGET, 'harness.rs'})
        for entry in manifest['files']:
            self.assertEqual(Path(entry['fixture']).name, entry['fixture'])
            data = (FIXTURES / entry['fixture']).read_bytes()
            self.assertEqual(len(data), entry['bytes'])
            self.assertEqual(hashlib.sha256(data).hexdigest(), entry['sha256'])
        self.assertEqual(hashlib.sha256(RAW.read_bytes()).hexdigest(), BASELINE_SHA256)

    def test_only_free_list_selection_changes_and_linked_delivery_matches(self):
        original = RAW.read_text(encoding='utf-8')
        fixed = apply_grass_pool_fixes(TARGET, original)
        self.assertEqual(fixed.replace(FIXED, ORIGINAL, 1), original)
        self.assertEqual(fixed.count(FIXED), 1)
        self.assertEqual(apply_grass_pool_fixes(TARGET, fixed), fixed)
        linked = LINKED.read_text(encoding='utf-8')
        self.assertEqual(fixed.replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).'), linked)
        self.assertEqual(apply_grass_pool_fixes(TARGET, linked), linked)

    def test_pointer_borrowed_and_crlf_inputs_keep_their_original_form(self):
        borrowed = RAW.read_text(encoding='utf-8')
        pointer = borrowed.replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).')
        for source in (borrowed, pointer):
            for newline in ('\n', '\r\n'):
                value = source.replace('\n', newline)
                fixed = apply_grass_pool_fixes(TARGET, value)
                self.assertEqual(fixed.count(newline), value.count(newline))
                self.assertNotIn('\n', fixed.replace(newline, ''))
                self.assertEqual(apply_grass_pool_fixes(TARGET, fixed), fixed)

    def test_other_targets_and_all_lifetime_shape_drift_fail_closed(self):
        source = RAW.read_text(encoding='utf-8')
        self.assertEqual(apply_grass_pool_fixes('other.rs', source), source)
        for broken in (source + source, source.replace('23333368i64', '23333372i64', 1),
                       source.replace('3i32', '2i32', 1), source.replace('ph: &mut Phantom', 'ph: &Phantom'),
                       source.replace('4096i32', '2048i32'), source.replace(ORIGINAL, ORIGINAL + '\n' + ORIGINAL),
                       apply_grass_pool_fixes(TARGET, source).replace('(r38 as u32) >= 3', '(r38 as u32) >= 2')):
            with self.assertRaises(ValueError):
                apply_grass_pool_fixes(TARGET, broken)

    def test_linker_regenerates_the_selection_once_without_abi_symbols(self):
        tree = ast.parse((ROOT / 'tools/link_engine.py').read_text(encoding='utf-8'))
        calls = [node for node in ast.walk(tree) if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == 'apply_grass_pool_fixes']
        self.assertEqual(len(calls), 1)
        self.assertEqual(ast.unparse(calls[0]), 'apply_grass_pool_fixes(source.name, text)')


if __name__ == '__main__':
    unittest.main()
