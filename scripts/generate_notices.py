"""Generate versioned attribution and preserve packaged dependency notices.

Run cargo metadata --locked --format-version 1 > target/license-metadata.json
before this script. Only notices are copied; no dependency source is vendored.
"""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[1]
metadata = json.loads((ROOT / 'target/license-metadata.json').read_text(encoding='utf-8-sig'))
packages = sorted((p for p in metadata['packages'] if p['source']), key=lambda p: (p['name'], p['version']))
out = ['# Third-party dependency notices', '',
       'Generated from Cargo.lock and cargo metadata. This inventory includes all workspace targets,',
       'build dependencies and optional platform dependencies, not only the installed game binaries.',
       'Original notices below retain their respective copyrights and license terms.', '',
       'See CREDITS.md for upstream code, controller mappings and external toolchains.', '',
       '| Package | Version | Declared license | Authors | Notice sections |',
       '| --- | --- | --- | --- | --- |']
notices = {}
missing = []
for p in packages:
    root = Path(p['manifest_path']).parent
    paths = [q for q in root.rglob('*') if q.is_file() and
             (q.name.upper().startswith(('LICENSE', 'LICENCE', 'COPYING', 'NOTICE', 'COPYRIGHT'))
              or q.name.upper() in {'OFL.TXT', 'UFL.TXT', 'OFL', 'UFL'})]
    downloaded = ROOT / 'target/upstream-notices' / f"{p['name']}-{p['version']}"
    if downloaded.is_dir():
        paths.extend(q for q in downloaded.iterdir() if q.is_file())
    if p.get('license_file'):
        explicit = root / p['license_file']
        if explicit.is_file() and explicit not in paths:
            paths.append(explicit)
    refs = []
    for path in sorted(paths):
        text = path.read_text(encoding='utf-8', errors='replace')
        digest = hashlib.sha256(text.encode()).hexdigest()[:16]
        if digest not in notices:
            notices[digest] = {'text': text, 'owners': []}
        label = path.relative_to(root).as_posix() if path.is_relative_to(root) else 'upstream ' + path.name
        notices[digest]['owners'].append(f"{p['name']} {p['version']}: {label}")
        refs.append(f'[notice](#notice-{digest})')
    if not paths:
        missing.append(f"{p['name']} {p['version']} ({p.get('license')})")
    authors = ', '.join(p.get('authors', [])) or 'See package source'
    clean = lambda value: str(value).replace('|', '\\|').replace('\n', ' ')
    out.append(f"| [{p['name']}](https://crates.io/crates/{p['name']}/{p['version']}) | {p['version']} | {clean(p.get('license') or 'License file')} | {clean(authors)} | {', '.join(dict.fromkeys(refs)) or 'No notice file packaged; see source link'} |")
for digest, notice in notices.items():
    out.extend(['', f'## Notice {digest}', '', '\n'.join('- ' + owner for owner in notice['owners']), '',
                '````text', notice['text'].rstrip(), '````'])
(ROOT / 'Licenses').mkdir(exist_ok=True)
(ROOT / 'Licenses/THIRD_PARTY_NOTICES.md').write_text('\n'.join(out) + '\n', encoding='utf-8')
print(f'NOTICES packages={len(packages)} unique_texts={len(notices)} no_packaged_notice={len(missing)}')
for entry in missing:
    print('NO_PACKAGED_NOTICE ' + entry)
