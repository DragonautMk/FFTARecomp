"""Summarize camera benchmark runs and compare deterministic checkpoints."""
import argparse
import csv
from pathlib import Path
import re
import statistics

def load(directory):
    rows = list(csv.DictReader((directory / 'benchmark-camera.csv').open(encoding='utf-8-sig')))
    previous = None
    moving = []
    for row in rows:
        position = (row['hofs'], row['vofs'])
        if int(row['frame']) >= 9700 and position != previous and position[0] != '-1':
            moving.append(float(row['frame_ms']))
        previous = position
    if not moving:
        raise ValueError(f'{directory}: no measured camera movement')
    values = sorted(moving)
    hashes = (directory / 'benchmark-hashes.txt').read_text(encoding='utf-8')
    if not hashes.rstrip().endswith('COMPLETE'):
        raise ValueError(f'{directory}: incomplete replay')
    print(f'{directory}: moving={len(values)} mean={statistics.mean(values):.3f} ms '
          f'p95={values[int((len(values)-1)*.95)]:.3f} ms max={max(values):.3f} ms')
    if 'audio_underruns' in rows[0]:
        final = [r for r in rows if int(r['frame']) >= 9700]
        print('  Audio underruns: total=' + rows[-1]['audio_underruns'] +
              ' camera=' + str(int(final[-1]['audio_underruns']) - int(final[0]['audio_underruns'])))
    checkpoints = [re.findall(label + r':\s*([0-9a-f]+)', hashes)
                   for label in ['Framebuffer FNV64', 'OAM FNV64']]
    if any(len(group) != 4 for group in checkpoints):
        raise ValueError(f'{directory}: expected four checkpoints')
    trajectory = [(r['frame'], r['hofs'], r['vofs'], r['keys']) for r in rows]
    return checkpoints, trajectory

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('runs', type=Path, nargs='+')
    args = parser.parse_args()
    reference = None
    for directory in args.runs:
        current = load(directory)
        if reference is not None:
            if current != reference:
                raise SystemExit('FAIL: framebuffer/OAM checkpoints or camera trajectory differ')
            print('  Framebuffer/OAM checkpoints and full camera trajectory: IDENTICAL')
        else:
            reference = current
    print('CPU timings are not display FPS; profiled runs are not fair speed comparisons.')

if __name__ == '__main__':
    main()
