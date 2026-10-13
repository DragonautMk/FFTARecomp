from pathlib import Path
import re

root=Path('target/ffta-zoom')
def lines(name):
    raw=(root/name).read_bytes()
    return raw.decode('utf-16' if raw.startswith(b'\xff\xfe') else 'utf-8',errors='replace').splitlines()
def eq(name):
    return {int(f):(fb,oam) for f,fb,oam in re.findall(r'FFTA_EQ frame=(\d+) framebuffer=(\w+) oam=(\w+)', '\n'.join(lines(name)))}
a,b=eq('snow-guard-extra0-off.log'),eq('snow-guard-extra0-on.log')
assert set(a)==set(b)==set(range(1,601))
assert a==b
print('EXTRA0 frames=600 framebuffer_and_oam_equal=True')
def characters(name):
    return {int(re.search(r'frame=(\d+)',s)[1]):s for s in lines(name) if s.startswith('FFTA_CHARACTERS ') and 11340<=int(re.search(r'frame=(\d+)',s)[1])<=12540}
a,b=characters('stage-on.log'),characters('sky-extended.log')
assert set(a)==set(b)==set(range(11340,12541))
changed=[f for f in a if a[f]!=b[f]]
print('CHARACTERS frames=1201 changed='+str(len(changed)))
assert not changed
data=(root/'sky-extended-13101.ppm').read_bytes()
header,pixels=data.split(b'\n255\n',1)
width,height=map(int,header.splitlines()[1].split())
pixel=lambda y:tuple(pixels[y*width*3:y*width*3+3])
extra_y=(height-160)//2
assert pixel(0)!=pixel(extra_y)
assert pixel(height-1)!=pixel(extra_y+159)
for y in [0,extra_y-1,extra_y,extra_y+159,extra_y+160,height-1]:
    print(f'SKY row={y} rgb={pixel(y)}')
