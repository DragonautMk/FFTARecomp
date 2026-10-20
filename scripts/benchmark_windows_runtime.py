"""Compare two Windows runtimes using one existing native ROM translation.

Runs on Windows directly, or on Linux with --wine wine64 --wine-prefix PATH.
No save is written. Hash capture and timing runs are separate so hashing does
not distort the measured emulation cost. ROMs and logs stay outside sources.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import statistics
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--rom', type=Path, required=True)
    parser.add_argument('--library', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    inputs = parser.add_mutually_exclusive_group()
    inputs.add_argument('--input', type=Path)
    inputs.add_argument('--demo', action='store_true',
                        help='generate the runtime\'s deterministic Start/A replay')
    parser.add_argument('--sav', type=Path)
    parser.add_argument('--frames', type=int, default=6000)
    parser.add_argument('--warmup', type=int, default=120)
    parser.add_argument('--runs', type=int, default=3)
    parser.add_argument('--widescreen', default='16:9')
    parser.add_argument('--wine')
    parser.add_argument('--wine-prefix', type=Path)
    args = parser.parse_args()
    if args.frames <= args.warmup or args.warmup < 0 or args.runs < 1:
        parser.error('require frames > warmup >= 0 and runs >= 1')
    for path in (args.baseline, args.candidate, args.rom, args.library,
                 args.input, args.sav):
        if path is not None and not path.is_file():
            parser.error(f'file not found: {path}')
    args.out.mkdir(parents=True, exist_ok=True)
    if args.demo:
        args.input = args.out / 'demo.input'
        rows = ['gba-input v1']
        previous = None
        for frame in range(args.frames + 1):
            phase = frame % 180
            keys = 0x3ff
            if frame >= 240:
                if phase < 10:
                    keys &= ~(1 << 3)
                elif 60 <= phase < 70:
                    keys &= ~1
            if keys != previous:
                rows.append(f'{frame} {keys:03x}')
                previous = keys
        args.input.write_text('\n'.join(rows) + '\n', encoding='utf-8')
    env = os.environ.copy()
    env['FFTA_BENCH_LIBRARY'] = str(args.library.resolve())
    env['FFTA_WIDESCREEN'] = args.widescreen
    if args.wine_prefix:
        env['WINEPREFIX'] = str(args.wine_prefix.resolve())
    if args.wine:
        env['WINEDEBUG'] = '-all'

    def run(label, exe, capture):
        run_env = env.copy()
        run_env['FFTA_PATCH_VERIFY_FRAMES'] = '1' if capture else '0'
        # This warm-up mode temporarily disables sprite expansion before the
        # measured interval. Use the same mode for correctness and timing.
        run_env['FFTA_BENCH_FROM'] = str(args.warmup)
        command = ([args.wine] if args.wine else []) + [
            str(exe.resolve()), 'runc', str(args.rom.resolve()),
            '--frames', str(args.frames),
        ]
        if args.input:
            command += ['--input', str(args.input.resolve())]
        if args.sav:
            command += ['--sav', str(args.sav.resolve())]
        log = args.out / f'{label}.log'
        with log.open('w', encoding='utf-8') as output:
            subprocess.run(command, env=run_env, stdout=output,
                           stderr=subprocess.STDOUT, check=True)
        text = log.read_text(encoding='utf-8')
        end = re.search(r'frames: (\d+) hash: ([0-9a-f]+) '
                        r'native_blocks: (\d+) fallback_steps: (\d+)', text)
        timing = re.search(r'FFTA_BENCH .*ms_per_frame=([0-9.]+)', text)
        if not end or int(end[1]) != args.frames or not timing:
            raise RuntimeError(f'incomplete run: {log}')
        hashes = {int(f): (fb, oam) for f, fb, oam in re.findall(
            r'FFTA_EQ frame=(\d+) framebuffer=([0-9a-f]+) oam=([0-9a-f]+)', text)}
        if capture and set(hashes) != set(range(1, args.frames + 1)):
            raise RuntimeError(f'incomplete frame capture: {log}')
        print(f'{label}: {timing[1]} ms/frame', flush=True)
        return tuple(end.groups()), float(timing[1]), hashes

    reference, _, hashes = run('baseline-equivalence', args.baseline, True)
    candidate, _, candidate_hashes = run('candidate-equivalence', args.candidate, True)
    if candidate != reference or candidate_hashes != hashes:
        raise RuntimeError('framebuffer/OAM or execution counts differ')
    print(f'Equivalent framebuffer/OAM on all {args.frames} frames.', flush=True)
    samples = {'baseline': [], 'candidate': []}
    for i in range(args.runs):
        # Alternate ordering to reduce temperature / background-load bias.
        order = [('baseline', args.baseline), ('candidate', args.candidate)]
        if i % 2:
            order.reverse()
        for name, exe in order:
            state, ms, _ = run(f'{name}-{i + 1}', exe, False)
            if state != reference:
                raise RuntimeError('timing run diverged from equivalence capture')
            samples[name].append(ms)
    before = statistics.median(samples['baseline'])
    after = statistics.median(samples['candidate'])
    summary = {
        'platform': 'Windows via Wine' if args.wine else 'Windows',
        'frames': args.frames, 'warmup': args.warmup,
        'widescreen': args.widescreen, 'runs': args.runs,
        'framebuffer_oam_equal': True,
        'final_state': reference,
        'samples_ms_per_frame': samples,
        'median_baseline_ms': before, 'median_candidate_ms': after,
        'emulation_time_reduction_percent': (1 - after / before) * 100,
        'note': 'Headless CPU/work timings; not displayed FPS or an AI-turn benchmark.',
        'sha256': {name: hashlib.sha256(path.read_bytes()).hexdigest()
                   for name, path in [('baseline', args.baseline),
                                      ('candidate', args.candidate),
                                      ('rom', args.rom), ('library', args.library)]},
    }
    (args.out / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, indent=2))


if __name__ == '__main__':
    main()
