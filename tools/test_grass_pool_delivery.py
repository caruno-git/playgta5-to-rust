import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from grass_pool_fixes import TARGET

__author__ = 'carunodev @stfu'
ROOT = Path(__file__).resolve().parent.parent
FIXTURES = ROOT / 'tools/testdata/grass_pool'
LINKED = ROOT / 'linked/generated/gen011/src' / TARGET


@unittest.skipUnless(shutil.which('rustc'), 'rustc required for actual grass pool allocator execution')
class GrassPoolDeliveryTests(unittest.TestCase):
    def test_actual_pool_preserves_retirement_and_completion_under_reuse_exhaustion_and_wrap(self):
        flags = subprocess.BELOW_NORMAL_PRIORITY_CLASS | subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0
        reports = {}
        harness = (FIXTURES / 'harness.rs').read_text(encoding='utf-8').replace('__NUMERIC__', str(ROOT / 'wasm-rt/src/numeric.rs'))
        with tempfile.TemporaryDirectory(prefix='playgta5-grass-pool-') as directory:
            temporary = Path(directory)
            (temporary / 'harness.rs').write_bytes(harness.encode('utf-8'))
            for variant, source in [('original', FIXTURES / TARGET), ('fixed', LINKED)]:
                text = source.read_bytes().decode('utf-8').replace('ph: &mut Phantom', 'ph: *mut Phantom').replace('ph.', '(*ph).')
                (temporary / 'pool.rs').write_bytes(text.encode('utf-8'))
                binary = temporary / ('fixture.exe' if os.name == 'nt' else 'fixture')
                built = subprocess.run(['rustc', '--edition=2021', '-Awarnings', str(temporary / 'harness.rs'), '-o', str(binary)],
                                       capture_output=True, text=True, timeout=60, creationflags=flags)
                self.assertEqual(built.returncode, 0, built.stdout + built.stderr)
                checked = subprocess.run([str(binary)], capture_output=True, text=True, timeout=30, creationflags=flags)
                self.assertEqual(checked.returncode, 0, checked.stdout + checked.stderr)
                reports[variant] = [json.loads(line) for line in checked.stdout.splitlines()]
        self.assertEqual(len(reports['original']), 4)
        self.assertEqual(len(reports['fixed']), 4)
        for original, fixed in zip(reports['original'], reports['fixed']):
            for key in ('case', 'frames', 'allocations_requested', 'new_resources', 'logical_completion_checksum', 'links_counts_canaries_preserved', 'no_early_reuse'):
                self.assertEqual(original[key], fixed[key], (key, original, fixed))
            self.assertGreaterEqual(fixed['minimum_reuse_gap'], 3)
        self.assertEqual([reports[key][0]['distinct_resources'] for key in ('original', 'fixed')], [3000, 240])
        print(json.dumps({'grass_pool': reports, 'scope': 'Actual translated allocator with simulated resource initialization and GPU completion; not live gameplay'}, sort_keys=True))


if __name__ == '__main__':
    unittest.main()
