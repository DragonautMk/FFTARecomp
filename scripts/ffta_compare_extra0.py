from pathlib import Path
import re

root = Path('target/ffta-zoom')
def frames(name):
    data = (root / name).read_bytes()
    text = data.decode('utf-16' if data.startswith(b'\xff\xfe') else 'utf-8', errors='replace')
    return dict((int(f), (fb, oam)) for f, fb, oam in re.findall(
        r'FFTA_EQ frame=(\d+)\s+framebuffer=([0-9a-f]+)\s+oam=([0-9a-f]+)', text))
a, b = frames('eq-off.log'), frames('eq-on.log')
assert set(a) == set(b) == set(range(1, 601)), 'Missing frames'
changed = [f for f in a if a[f] != b[f]]
print(f'EXTRA0 frames=600 framebuffer_and_oam_equal={not changed}')
assert not changed, changed
