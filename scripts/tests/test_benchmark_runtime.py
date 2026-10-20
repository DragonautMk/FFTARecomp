"""Behavioral checks for benchmark rejection and input/output separation."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('benchmark', Path(__file__).parents[1] / 'benchmark_windows_runtime.py')
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)


class BenchmarkTests(unittest.TestCase):
    def log(self, capture=True):
        rows = [f'FFTA_BENCH_SCENE frame=1 guest_sha256={"a" * 64} sprite_requested=true',
                'FFTA_BENCH first=1 last=3 frames=2 seconds=0.004000 ms_per_frame=2.000000 width=284',
                'frames: 3 hash: 0123456789abcdef native_blocks: 50 fallback_steps: 100']
        if capture:
            rows += [f'FFTA_EQ frame={i} framebuffer={"b" * 64} oam={"c" * 64}' for i in range(1, 4)]
        return '\n'.join(rows)

    def test_requires_complete_unique_frame_capture(self):
        text = self.log()
        self.assertEqual(len(bench.parse_run(text, 3, 1, capture=True)['hashes']), 3)
        for broken in (text.rsplit('\n', 1)[0], text + '\n' + text.splitlines()[-1],
                       text.replace('last=3', 'last=2'), text.replace('first=1', 'first=0')):
            with self.subTest(broken=broken), self.assertRaises(ValueError):
                bench.parse_run(broken, 3, 1, capture=True)

    def test_timing_cannot_contain_hash_capture_or_zero_time(self):
        with self.assertRaises(ValueError):
            bench.parse_run(self.log(), 3, 1)
        with self.assertRaises(ValueError):
            bench.parse_run(self.log(False).replace('ms_per_frame=2.000000', 'ms_per_frame=0'), 3, 1)
        with self.assertRaises(ValueError):
            bench.parse_run(self.log(False).replace('seconds=0.004000', 'seconds=1e999'), 3, 1)

    def test_machine_state_and_warmup_divergence_are_rejected(self):
        text = self.log() + '\nFFTA_FINAL_STATE sha256=' + 'd' * 64
        reference = bench.parse_run(text, 3, 1, capture=True, verify_state=True)
        for wrong in (dict(reference, state='e' * 64), dict(reference, scene=('1', 'f' * 64, 'true'))):
            with self.assertRaises(ValueError):
                bench.assert_equivalent(reference, wrong)
        with self.assertRaises(ValueError):
            bench.parse_run(self.log(), 3, 1, capture=True, verify_state=True)

    def test_translation_changes_can_change_dispatch_counts(self):
        reference = bench.parse_run(self.log(), 3, 1, capture=True)
        different = dict(reference, native_blocks=300, fallback_steps=0)
        bench.assert_equivalent(reference, different)
        with self.assertRaises(ValueError):
            bench.assert_equivalent(reference, different, require_counts=True)

    def test_profile_must_be_complete_ordered_and_finite(self):
        line = 'FFTA_FRAME_TIMES samples=2 omitted=0 p50_ms=1 p95_ms=3 p99_ms=3 max_ms=3 over_budget=0 over_double_budget=0'
        text = self.log(False) + '\n' + line
        self.assertEqual(bench.parse_run(text, 3, 1, profile=True)['frame_times']['p99_ms'], 3)
        for broken in (text.replace('samples=2', 'samples=1'), text.replace('omitted=0', 'omitted=1'),
                       text.replace('p99_ms=3', 'p99_ms=nan'), text.replace('p99_ms=3', 'p99_ms=5'),
                       text.replace('over_double_budget=0', 'over_double_budget=1')):
            with self.subTest(broken=broken), self.assertRaises(ValueError):
                bench.parse_run(broken, 3, 1, profile=True)

    def test_target_platform_uses_binary_format(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'recomp.exe'
            for magic, expected in [(b'\x7fELF', 'Linux'), (b'MZ00', 'Windows'),
                                    (b'\xcf\xfa\xed\xfe', 'Darwin')]:
                path.write_bytes(magic)
                self.assertEqual(bench.binary_platform(path), expected)

    def test_command_refuses_output_over_input_even_through_symlink(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            exe = root / 'recomp'; exe.write_bytes(b'\x7fELF')
            rom = root / 'private.gba'; rom.write_bytes(b'private fixture')
            out = root / 'output'; out.mkdir()
            (out / 'baseline-equivalence.log').symlink_to(rom)
            with patch.object(bench.platform, 'system', return_value='Linux'), self.assertRaises(SystemExit):
                bench.main(['--baseline', str(exe), '--candidate', str(exe), '--rom', str(rom),
                            '--library', str(exe), '--out', str(out), '--demo'])
            self.assertEqual(rom.read_bytes(), b'private fixture')
            self.assertFalse((out / 'summary.json').exists())

    def test_failure_replaces_stale_success_summary(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            exe = root / 'recomp'; exe.write_bytes(b'\x7fELF')
            out = root / 'output'; out.mkdir()
            (out / 'summary.json').write_text('{"status":"passed"}')
            with patch.object(bench.platform, 'system', return_value='Linux'), \
                    patch.object(bench.subprocess, 'run', side_effect=ValueError('incomplete replay')), \
                    self.assertRaises(ValueError):
                bench.main(['--baseline', str(exe), '--candidate', str(exe), '--rom', str(exe),
                            '--library', str(exe), '--out', str(out), '--demo', '--frames', '3', '--warmup', '1'])
            self.assertIn('"status": "failed"', (out / 'summary.json').read_text())


if __name__ == '__main__':
    unittest.main()
