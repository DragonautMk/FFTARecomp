"""Fetch only upstream root license files pinned to packaged VCS commits."""
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from urllib.request import urlopen
from urllib.parse import urlparse
import json

root = Path(__file__).resolve().parents[1]
packages = json.loads((root / 'target/license-metadata.json').read_text(encoding='utf-8-sig'))['packages']
groups = {}
for p in packages:
    if not p['source']:
        continue
    source = Path(p['manifest_path']).parent
    downloaded = root / 'target/upstream-notices' / f"{p['name']}-{p['version']}"
    if downloaded.is_dir() and any(downloaded.iterdir()):
        continue
    if any(q.is_file() and q.name.upper().startswith(('LICENSE', 'LICENCE', 'COPYING', 'NOTICE')) for q in source.rglob('*')):
        continue
    vcs = source / '.cargo_vcs_info.json'
    repo = p.get('repository')
    if not repo:
        continue
    sha = json.loads(vcs.read_text())['git']['sha1'] if vcs.exists() else 'v' + p['version']
    url = urlparse(repo.removesuffix('.git'))
    parts = url.path.strip('/').split('/')
    if url.netloc == 'github.com' and len(parts) >= 2:
        base = f'https://raw.githubusercontent.com/{parts[0]}/{parts[1]}/{sha}/'
    elif url.netloc == 'gitlab.com' and len(parts) >= 2:
        base = f'https://gitlab.com/{parts[0]}/{parts[1]}/-/raw/{sha}/'
    else:
        continue
    groups.setdefault(base, []).append(p)

def fetch(item):
    base, packages = item
    count = 0
    names = ['LICENSE', 'LICENSE-MIT', 'LICENSE-APACHE', 'LICENSE.txt', 'LICENSE.md', 'COPYING', 'NOTICE', 'LICENSE.MIT', 'LICENSE.APACHE', 'LICENSE.BSD']
    if 'raw.githubusercontent.com' in base:
        parts = urlparse(base).path.strip('/').split('/')
        api = f'https://api.github.com/repos/{parts[0]}/{parts[1]}/git/trees/{parts[2]}'
        try:
            with urlopen(api, timeout=10) as response:
                tree = json.load(response)['tree']
            for entry in tree:
                if entry['path'].upper().startswith(('LICENSE', 'COPYING', 'NOTICE', 'COPYRIGHT')):
                    if entry['type'] == 'blob':
                        names.append(entry['path'])
                    elif entry['type'] == 'tree':
                        with urlopen(entry['url'], timeout=10) as response:
                            children = json.load(response)['tree']
                        names.extend(entry['path'] + '/' + child['path'] for child in children if child['type'] == 'blob')
        except Exception:
            pass
    for name in dict.fromkeys(names):
        try:
            with urlopen(base + name, timeout=10) as response:
                data = response.read(1024 * 1024)
            if b'<html' in data[:1000].lower() or not data:
                continue
        except Exception:
            continue
        for p in packages:
            dest = root / 'target/upstream-notices' / f"{p['name']}-{p['version']}"
            dest.mkdir(parents=True, exist_ok=True)
            (dest / name.replace('/', '_')).write_bytes(data)
            count += 1
    return count

with ThreadPoolExecutor(max_workers=12) as pool:
    counts = list(pool.map(fetch, groups.items()))
print(f'UPSTREAM_NOTICES repositories={len(groups)} files={sum(counts)}')
