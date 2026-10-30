#!/usr/bin/env python3
"""Package desktop portable runtimes, selecting only distributable files."""
import argparse
import hashlib
import json
import os
import posixpath
import re
from urllib.parse import unquote
from pathlib import Path
import tempfile
import tarfile
import zipfile

ROOT = Path(__file__).resolve().parent.parent


# Explicit public inputs only. Never enumerate the checkout, Data or runtime-dir.
BASE_DOCS = ('portable', 'docker', 'fps_overlay', 'menu_shortcuts', 'save_states',
             'save_backups', 'rom_data', 'rom_data_runtime', 'menu_clan_atlas',
             'extended_saves')
DEBUG_DOCS = ('debug_tools', 'debug_diagnostics', 'debug_sessions', 'debug_catalog',
              'debug_recordings', 'debug_replays', 'debug_rom_audit', 'debug_captures',
              'debug_comparisons', 'debug_control', 'debug_probes', 'debug_graphics',
              'debug_ui', 'debug_audio', 'debug_code', 'debug_issues', 'debug_gameplay',
              'debug_artifacts', 'debug_recent', 'debug_campaigns', 'report_home',
              'debug_operations', 'debug_content', 'debug_selector', 'debug_events')
DEVELOPMENT_DOCS = ('LOCAL_ARTIFACTS', 'MEMORY_WORKFLOW', 'DEBUG_TOOLS_TODO')
TOOL_FILES = (
    'scripts/artifact_coordinator.py', 'scripts/local_artifacts.py',
    'scripts/debug_artifacts.py', 'scripts/debug_artifacts.html',
    'scripts/debug_campaign.py', 'scripts/debug_campaign.html',
    'scripts/report_home.py', 'scripts/report_home.html',
    'scripts/linux/debug_artifacts.sh', 'scripts/linux/debug_campaign.sh',
    'scripts/linux/report_home.sh', 'scripts/linux/debug_runtime.sh',
    'scripts/windows/debug_artifacts.ps1', 'scripts/windows/debug_campaign.ps1',
    'scripts/windows/report_home.ps1', 'scripts/windows/debug_runtime.ps1',
)
LICENSE_FILES = (
    'LICENSE', 'LICENSE-MIT', 'LICENSE-APACHE', 'LICENSE-CC0', 'CREDITS.md',
    'THIRD_PARTY_NOTICES.md', 'LICENSE-gamecontrollerdb.txt', 'gamedb.sqlite.license',
    'README.md', 'ANDROID.md', 'TinyCC/COPYING', 'TinyCC/LICENSE-ADAPTER-MIT.txt',
    'fonts/Cinzel-OFL.txt', 'fonts/Inter-OFL.txt',
)
DOC_FILES = tuple('docs/' + name + '.md' for name in BASE_DOCS + DEBUG_DOCS) + tuple(
    'docs/development/' + name + '.md' for name in DEVELOPMENT_DOCS)


def safe_source(path):
    path = Path(os.path.abspath(path))
    for parent in reversed(path.parents):
        if parent.is_symlink() or not parent.is_dir() or getattr(parent.stat(), 'st_file_attributes', 0) & 0x400:
            raise SystemExit(f'Nonordinary package input ancestor: {parent}')
    if path.is_symlink() or not path.is_file() or getattr(path.stat(), 'st_file_attributes', 0) & 0x400:
        raise SystemExit(f'Missing regular package input: {path}')
    return path


def bundled_document(name, content, names):
    """Keep bundled links live; mark repository-only evidence as plain references."""
    text = content.decode('utf-8')
    changed = False
    def link(match):
        nonlocal changed
        marker, label, destination = match.groups()
        if destination.startswith(('#', 'http://', 'https://', 'mailto:')):
            return match.group(0)
        target = unquote(destination.split('#')[0])
        resolved = posixpath.normpath(posixpath.join(posixpath.dirname(name), target))
        if resolved in names:
            return match.group(0)
        changed = True
        return label + ' (riferimento nel checkout: ' + destination + ')'
    text = re.sub(r'(!?)\[([^\]\n]+)\]\(([^)\n]+)\)', link, text)
    if changed:
        text = '> Copia distribuita: i riferimenti a prove/sorgenti non incluse sono indicati come testo del checkout.\n\n' + text
    return text.encode('utf-8')


def main(default_platform="windows", argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--platform', choices=('windows', 'linux'), default=default_platform)
    parser.add_argument('--runtime-dir', type=Path, default=ROOT / 'target/release')
    parser.add_argument('--output', type=Path, default=None)
    parser.add_argument('--private-prefix', action='append', default=[], help='Reject an embedded UTF-8/UTF-16LE build prefix; repeatable, no binary rewriting')
    args = parser.parse_args(argv)
    if any(not prefix or len(prefix)>4096 for prefix in args.private_prefix):
        parser.error('--private-prefix needs1..4096characters')
    forbidden_prefixes = [encoded for prefix in args.private_prefix for encoded in (prefix.encode('utf-8'), prefix.encode('utf-16-le'))]
    linux = args.platform == 'linux'
    extension = '.tar.gz' if linux else '.zip'
    if args.output is None:
        args.output = ROOT / ('dist/FFTARecomp-' + args.platform + '-portable' + extension)
    launcher = 'FFTARecomp' if linux else 'FFTARecomp.exe'
    runtime = 'recomp' if linux else 'recomp.exe'
    library = 'libtcc.so' if linux else 'libtcc.dll'
    files = {
        launcher: args.runtime_dir / ('ffta-play' if linux else 'ffta-play.exe'),
        runtime: args.runtime_dir / runtime,
        'tcc/' + library: args.runtime_dir / 'tcc' / library,
        'gamedb.sqlite': ROOT / 'gamedb.sqlite',
    }
    for name in ('COPYING', 'README.md', 'build.py', 'desktop_abi.c', 'libtcc.h', 'source.zip'):
        files['TinyCC/' + name] = ROOT / 'TinyCC' / name
    for name in LICENSE_FILES:
        files['Licenses/' + name] = ROOT / 'Licenses' / name
    for name in DOC_FILES + TOOL_FILES:
        files[name] = ROOT / name
    files = {name: safe_source(source) for name, source in files.items()}
    output = Path(os.path.abspath(args.output))
    if output in files.values() or output.is_symlink():
        raise SystemExit('Package output must not replace a selected input or symlink')
    prefix = 'FFTARecomp-portable'
    readme = '''FFTARecomp portable - {platform} x86_64
Estrai tutta la cartella in una posizione scrivibile e avvia {launch}.
Al primo avvio scegli la tua ROM FFTA USA/Europa: viene copiata come ffta.gba.
Puoi anche copiarla manualmente accanto agli eseguibili prima di avviare.
Nessuna ROM o BIOS e incluso nel pacchetto. Nessuna installazione richiesta.

Tutti i dati dell'app restano nella cartella Data. Per spostare il gioco,
copia TUTTA la cartella, inclusi Data, ffta.gba e portable.txt, a gioco chiuso.
Per importare progressi: a gioco chiuso copia il tuo .sav in Data/saves/ffta.sav.
Se esiste gia, conservane prima una copia. Dettagli: docs/portable.md.
Menu: Esc oppure Start+Back/Select sul controller; combo in Gioco > Controlli.
Savestate: Gioco > Salvataggi > Save states (9/99 slot, anteprima e annulla).
Backup del .sav: Gioco > Salvataggi > Save backups (le ultime 2 copie all'avvio).
Per errori di avvio consulta play-error.log nella cartella principale.
Guida debug: docs/debug_tools.md (bug, registrazione, confronto, letture, pulizia).
Launcher con log: scripts/windows/debug_runtime.ps1 oppure scripts/linux/debug_runtime.sh.
Strumenti offline Python e launcher PS/Bash/Docker sono inclusi; richiedono i
prerequisiti dichiarati nella guida. Nessun caso personale o risultato e incluso.
PACKAGE-MANIFEST.json identifica tutti i payload, non certifica la piattaforma.
'''.format(platform=args.platform, launch='./FFTARecomp' if linux else launcher)
    if linux:
        readme += '\nRichiede Ubuntu/Linux x86_64 con sessione grafica, ALSA e libudev.\n'
        readme += 'Non richiede Wine. Se il desktop chiede cosa fare, scegli Esegui.\n'
    generated = {
        'portable.txt': b'FFTARecomp portable mode. Keep beside both executables.\n',
        'LEGGIMI.txt': readme.encode('utf-8'),
        '.gitignore': b'/Data/\n/target/\n/ffta.gba\n/gba_bios.bin\n/index.html\n/*.log\n',
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    # Stage only selected inputs. Existing personal Data is never copied.
    with tempfile.TemporaryDirectory(dir=args.output.parent) as staging:
        directory = Path(staging) / prefix
        directory.mkdir()
        payloads = {}
        for name, source in files.items():
            data = source.read_bytes()
            if len(data) >= 192 and data[0xB2] == 0x96 and data[0xAC:0xB0] in (b'AFXE', b'AFXP'):
                raise SystemExit(f'Game ROM header in package input: {name}')
            distributed = bundled_document(name, data, set(files) | set(generated)) if name.startswith('docs/') and name.endswith('.md') else data
            if any(prefix in distributed for prefix in forbidden_prefixes):
                raise SystemExit(f'Private build prefix in package input: {name}; rebuild with compiler path remapping')
            payloads[name] = {'sha256': hashlib.sha256(distributed).hexdigest(),
                              'source_sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(distributed)}
            dest = directory / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_bytes(distributed)
            dest.chmod(0o755 if name in (launcher, runtime) or name.endswith('.sh') else 0o644)
        for name, content in generated.items():
            (directory / name).write_bytes(content)
            payloads[name] = {'sha256': hashlib.sha256(content).hexdigest(), 'bytes': len(content)}
        manifest = {'schema': 1, 'kind': 'ffta-portable-package', 'platform': args.platform, 'private_prefix_checks': len(args.private_prefix),
                    'files': payloads,
                    'scope': 'Exact package payload identities; no personal Data, ROM/BIOS/saves/recordings/generated game code or tests. Platform validation is declared in docs/debug_tools.md, never inferred from packaging.'}
        (directory / 'PACKAGE-MANIFEST.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
        for name in ('config', 'cache', 'saves'):
            (directory / 'Data' / name).mkdir(parents=True)
        archive = Path(staging) / ('portable' + extension)
        if linux:
            with tarfile.open(archive, 'w:gz') as bundle:
                bundle.add(directory, arcname=prefix, filter=clean_tar_metadata)
            with tarfile.open(archive, 'r:gz') as bundle:
                for path in directory.rglob('*'):
                    if path.is_file() and bundle.extractfile(prefix + '/' + path.relative_to(directory).as_posix()).read() != path.read_bytes():
                        raise SystemExit(f'Archive input mismatch: {path.name}')
                for name in (launcher, runtime, *(name for name in TOOL_FILES if name.endswith('.sh'))):
                    if bundle.getmember(prefix + '/' + name).mode != 0o755:
                        raise SystemExit(f'Missing executable permission: {name}')
        else:
            with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as bundle:
                for path in sorted(directory.rglob('*')):
                    bundle.write(path, prefix + '/' + path.relative_to(directory).as_posix())
            with zipfile.ZipFile(archive) as bundle:
                if bundle.testzip() is not None:
                    raise SystemExit('Archive CRC validation failed')
                for path in directory.rglob('*'):
                    if path.is_file() and bundle.read(prefix + '/' + path.relative_to(directory).as_posix()) != path.read_bytes():
                        raise SystemExit(f'Archive input mismatch: {path.name}')
        archive.replace(args.output)
    print(f'Created {args.output} ({args.output.stat().st_size:,} bytes)')


def clean_tar_metadata(info):
    info.uid = info.gid = 0
    info.uname = info.gname = ''
    return info


if __name__ == '__main__':
    main()
