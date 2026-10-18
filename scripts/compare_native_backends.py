"""Compare every framebuffer/OAM hash, rejecting incomplete captures."""
from pathlib import Path
import argparse
import re

parser = argparse.ArgumentParser()
parser.add_argument('reference', type=Path)
parser.add_argument('candidate', type=Path)
parser.add_argument('--frames', type=int, default=600)
args = parser.parse_args()

def read(path):
    data = path.read_bytes()
    text = data.decode('utf-16' if data.startswith(b'\xff\xfe') else 'utf-8', errors='replace')
    return {int(f): (fb, oam) for f, fb, oam in re.findall(
        r'FFTA_EQ frame=(\d+)\s+framebuffer=([0-9a-f]+)\s+oam=([0-9a-f]+)', text)}

a, b = read(args.reference), read(args.candidate)
expected = set(range(1, args.frames + 1))
if set(a) != expected or set(b) != expected:
    raise SystemExit(f'INCOMPLETE expected={args.frames} reference={len(a)} candidate={len(b)}')
changed = [f for f in sorted(expected) if a[f] != b[f]]
print(f'NATIVE_EQ frames={args.frames} framebuffer_and_oam_equal={not changed}')
if changed:
    raise SystemExit(f'Different frames: {changed}')
