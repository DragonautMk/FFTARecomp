"""Analyse native tutorial character telemetry; never reads or changes a ROM."""
from pathlib import Path
import argparse,csv,re,json

parser=argparse.ArgumentParser()
parser.add_argument('directory',type=Path)
args=parser.parse_args()
summaries={};cases={};hashes={}
for case in ['current32','proposed32','original0','proposed0']:
    path=args.directory/(case+'.log')
    if case=='proposed32' and (args.directory/'proposed32-corrected.log').exists():path=args.directory/'proposed32-corrected.log'
    if not path.exists():continue
    raw=path.read_bytes()
    lines=raw.decode('utf-16' if raw.startswith(b'\xff\xfe') else 'utf-8',errors='replace').splitlines()
    rows=[];case_hashes={}
    for line in lines:
        if line.startswith('FFTA_CHARACTERS '):
            value=lambda key:re.search(r'\b'+key+r'=([^ ]+)',line)[1]
            def ids(key):
                match=re.search(r'\b'+key+r'=[\[{](.*?)[\]}]',line)
                if match is None:raise ValueError(f'incomplete metric at frame {frame}: {key}')
                return [int(v) for v in match[1].split(', ') if v]
            frame=int(value('frame'))
            present=ids('ids');visible=ids('visible_ids');oam=ids('oam_ids')
            duplicates=re.search(r'duplicate_groups=(\[.*?\])',line)[1]
            rows.append(dict(frame=frame,valid=value('roster_valid'),present=len(present),missing=8-len(present),
                missing_ids=' '.join(map(str,sorted(set(range(8))-set(present)))),visible=len(visible),
                visible_missing=8-len(visible),visible_missing_ids=' '.join(map(str,sorted(set(range(8))-set(visible)))),
                oam=len(oam),duplicate_groups=duplicates,npc_present=value('npc_present'),npc_pixels=int(value('npc_pixels'))))
        elif line.startswith('FFTA_FRAME_HASH '):
            frame=int(re.search(r'frame=(\d+)',line)[1])
            case_hashes[frame]=dict(re.findall(r'(framebuffer|wide|oam|obj_vram)=(\w+)',line))
    if not rows: print(case,'NO_MEDIDO metrics_not_yet_present');continue
    cases[case]={row['frame']:row for row in rows};hashes[case]=case_hashes
    for suffix,subset in [('frames',rows),('failures',[r for r in rows if r['missing']>0]),('visible-failures',[r for r in rows if r['visible_missing']>0])]:
        with (args.directory/f'{case}-{suffix}.csv').open('w',newline='',encoding='utf-8') as file:
            writer=csv.DictWriter(file,fieldnames=rows[0].keys());writer.writeheader();writer.writerows(subset)
    summary=dict(first=rows[0]['frame'],last=rows[-1]['frame'],frames=len(rows),
        invalid=sum(r['valid']!='true' for r in rows),present_min=min(r['present'] for r in rows),present_max=max(r['present'] for r in rows),
        failed=sum(r['missing']>0 for r in rows),visible_min=min(r['visible'] for r in rows),visible_max=max(r['visible'] for r in rows),
        visible_failed=sum(r['visible_missing']>0 for r in rows),duplicates=sum(r['duplicate_groups']!='[]' for r in rows),
        over_eight=sum(r['present']>8 for r in rows),npc_present_frames=sum(r['npc_present']=='true' for r in rows))
    summary['including_npc_max']=max(r['present']+(r['npc_present']=='true') for r in rows)
    summary['including_npc_over_eight']=sum(r['present']+(r['npc_present']=='true')>8 for r in rows)
    summary['including_npc_visible_max']=max(r['visible']+(r['npc_pixels']>0) for r in rows)
    summary['including_npc_visible_over_eight']=sum(r['visible']+(r['npc_pixels']>0)>8 for r in rows)
    summaries[case]=summary;print(case,json.dumps(summary,separators=(',',':')))
if 'original0' in cases and 'proposed0' in cases:
    common=sorted(cases['original0'].keys()&cases['proposed0'].keys())
    mismatches=[frame for frame in common if any(cases['original0'][frame][key]!=cases['proposed0'][frame][key] for key in ['valid','present','missing_ids','visible','visible_missing_ids','oam'])]
    samples=sorted(hashes['original0'].keys()&hashes['proposed0'].keys())
    hash_mismatches=[frame for frame in samples if hashes['original0'][frame]!=hashes['proposed0'][frame]]
    print('extra0_metric_comparison',len(common),'mismatch_frames',mismatches[:20])
    print('extra0_hash_comparison',len(samples),'mismatch_frames',hash_mismatches[:20])
    summaries['extra0']=dict(metric_frames=len(common),metric_mismatches=mismatches,hash_samples=len(samples),hash_mismatches=hash_mismatches)
(args.directory/'character-summary.json').write_text(json.dumps(summaries,indent=2),encoding='utf-8')
