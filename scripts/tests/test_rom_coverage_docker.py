"""Docker launcher checks; no ROM or running Docker daemon is needed."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


@unittest.skipUnless(os.name == 'posix' and shutil.which('bash'), 'requires Bash')
class DockerCoverageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='ffta coverage docker ')
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name).resolve()
        self.repo = self.directory / 'project checkout'
        self.script = self.repo / 'scripts/linux/rom_coverage_docker.sh'
        self.script.parent.mkdir(parents=True)
        shutil.copy2(ROOT / 'scripts/linux/rom_coverage_docker.sh', self.script)
        self.defaults = {
            'rom': 'rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba',
            'dataset': 'target/rom-data-rank-gates/windows-managed-europe/offline dataset',
            'translation': 'out/1194 - Final Fantasy Tactics Advance (E)(Surplus).dll',
            'evidence': 'docs/development/rom-coverage/evidence.json',
        }
        for key, name in self.defaults.items():
            path = self.repo / name
            if key == 'dataset':
                path = path / 'manifest.toml'
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('argument routing fixture, not game data')
        self.bin = self.directory / 'bin'
        self.bin.mkdir()
        # The child PATH deliberately contains no Python, Cargo or nm.
        for name in ('dirname', 'basename', 'mkdir', 'id'):
            (self.bin / name).symlink_to(shutil.which(name))
        docker = self.bin / 'docker'
        docker.write_text('''#!/bin/bash
printf '%s\\0' "$@" > "$DOCKER_TEST_LOG/$1.args"
if [[ "$1" == buildx ]]; then exit "${DOCKER_TEST_BUILD_STATUS:-0}"; fi
exit "${DOCKER_TEST_RUN_STATUS:-0}"
''')
        docker.chmod(0o755)
        self.environment = dict(os.environ, PATH=str(self.bin), DOCKER_TEST_LOG=str(self.directory))

    def run_script(self, *args, **environment):
        return subprocess.run([shutil.which('bash'), str(self.script), *map(str, args)],
                              cwd=self.directory, env=dict(self.environment, **environment),
                              text=True, capture_output=True)

    def call(self, name):
        path = self.directory / (name + '.args')
        return path.read_bytes().decode().rstrip('\0').split('\0') if path.exists() else None

    def test_defaults_from_unrelated_cwd_without_host_toolchain(self):
        result = self.run_script()
        self.assertEqual(result.returncode, 0, result.stderr)
        build, run = self.call('buildx'), self.call('run')
        self.assertEqual(build[build.index('--target') + 1], 'coverage')
        self.assertEqual(build[build.index('--file') + 1], str(self.repo / 'Dockerfile.linux'))
        self.assertEqual(build[-1], str(self.repo))
        for key, target in [('rom', '/input/source.gba'), ('dataset', '/input/dataset'),
                            ('translation', '/input/translation.dll'), ('evidence', '/input/evidence.json')]:
            self.assertIn(f'type=bind,source={self.repo / self.defaults[key]},target={target},readonly', run)
        output = self.repo / 'docs/development/rom-coverage'
        self.assertIn(f'type=bind,source={output},target=/output', run)
        self.assertEqual(run[run.index('--network') + 1], 'none')
        self.assertEqual(run[run.index('--user') + 1], f'{os.getuid()}:{os.getgid()}')
        self.assertNotIn('/var/run/docker.sock', ' '.join(run))
        self.assertIn(str(output / 'index.html'), result.stdout)

    def test_relative_overrides_and_absolute_output_preserve_spaces(self):
        args = []
        for key, old in self.defaults.items():
            path = self.repo / 'custom inputs' / Path(old).name
            source = self.repo / old
            if key == 'dataset':
                shutil.copytree(source, path)
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(source, path)
            args += ['--' + key, str(path.relative_to(self.repo))]
        output = self.directory / 'coverage output'
        result = self.run_script(*args, '--output', output)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(f'type=bind,source={output},target=/output', self.call('run'))
        self.assertIn(str(self.repo / 'custom inputs'), ' '.join(self.call('run')))

    def test_missing_dataset_or_input_stops_before_docker(self):
        for key, name in self.defaults.items():
            path = self.repo / name
            if key == 'dataset':
                path /= 'manifest.toml'
            data = path.read_bytes()
            path.unlink()
            with self.subTest(key=key):
                result = self.run_script()
                self.assertEqual(result.returncode, 2)
                self.assertIn(str(path), result.stderr)
                self.assertIsNone(self.call('buildx'))
            path.write_bytes(data)

    def test_failures_are_propagated_without_success_message(self):
        result = self.run_script(DOCKER_TEST_BUILD_STATUS='17')
        self.assertEqual(result.returncode, 17)
        self.assertIsNone(self.call('run'))
        self.assertNotIn('ROM coverage on host:', result.stdout)
        result = self.run_script(DOCKER_TEST_RUN_STATUS='23')
        self.assertEqual(result.returncode, 23)
        self.assertIsNotNone(self.call('run'))
        self.assertNotIn('ROM coverage on host:', result.stdout)

    def test_bad_arguments_and_csv_paths_stop_before_docker(self):
        for args in [('--unknown',), ('--output',), ('--rom', ''),
                     *[('--output', 'bad' + char + 'path') for char in (',', '"', '\n', '\r')]]:
            with self.subTest(args=args):
                self.assertEqual(self.run_script(*args).returncode, 2)
        self.assertIsNone(self.call('buildx'))


if __name__ == '__main__':
    unittest.main()
