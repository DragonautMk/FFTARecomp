"""Compare completed replay telemetry without modifying ROMs or saves."""
from pathlib import Path
import csv, re, sys
root=Path(sys.argv[1])
def read(name):
    raw=(root/name).read_bytes()
    return raw.decode('utf-16' if raw.startswith(b'\xff\xfe') else 'utf-8',errors='replace').splitlines()
def records(lines,prefix):
    return {int(re.search(r'frame=(\d+)',line)[1]):line for line in lines if line.startswith(prefix)}
off=read('stage-off.log');on=read('stage-on.log')
chars=[records(lines,'FFTA_CHARACTERS ') for lines in [off,on]]
common=sorted(chars[0].keys()&chars[1].keys())
changes=[frame for frame in common if chars[0][frame]!=chars[1][frame]]
print('CHARACTERS frames='+str(len(common))+' changed='+str(len(changes)))
print('CHARACTERS_FINAL '+chars[1][max(common)])
hashes=[records(lines,'FFTA_FRAME_HASH ') for lines in [off,on]]
get=lambda text,key:re.search(r'\b'+key+r'=(\w+)',text)[1]
samples=sorted(hashes[0].keys()&hashes[1].keys())
differences=[f for f in samples if any(get(hashes[0][f],k)!=get(hashes[1][f],k) for k in ['framebuffer','oam'])]
print('CENTRAL_OAM samples='+str(len(samples))+' changed='+str(len(differences)))
frames=[records(lines,'FFTA_MOVEMENT ') for lines in [off,on]]
rows=[]
for frame in sorted(frames[0].keys()&frames[1].keys()):
    a,b=frames[0][frame],frames[1][frame]
    rows.append(dict(frame=frame,off=get(a,'framebuffer_sha256'),on=get(b,'framebuffer_sha256'),
        off_lower_nonzero=get(a,'lower_rows_nonzero'),on_lower_nonzero=get(b,'lower_rows_nonzero')))
with (root/'stage-frame-comparison.csv').open('w',newline='') as file:
    writer=csv.DictWriter(file,fieldnames=rows[0].keys());writer.writeheader();writer.writerows(rows)
for frame in [12540,12801,13101,13200]:
    if frame in frames[1]:print(frames[1][frame])
print('FULL_VIEW changed='+str(sum(r['off']!=r['on'] for r in rows))+' frames='+str(len(rows)))
