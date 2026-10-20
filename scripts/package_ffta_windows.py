#!/usr/bin/env python3
"""Package the Windows portable runtime, selecting only distributable files."""
import argparse
from pathlib import Path
import tempfile
import zipfile

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--runtime-dir', type=Path, default=ROOT / 'target/release')
    parser.add_argument('--output', type=Path, default=ROOT / 'dist/FFTARecomp-windows-portable.zip')
    args = parser.parse_args()
    files = {
        'FFTARecomp.exe': args.runtime_dir / 'ffta-play.exe',
        'recomp.exe': args.runtime_dir / 'recomp.exe',
        'tcc/libtcc.dll': args.runtime_dir / 'tcc/libtcc.dll',
        'gamedb.sqlite': ROOT / 'gamedb.sqlite',
    }
    for name in ('COPYING', 'README.md', 'build.py', 'desktop_abi.c', 'libtcc.h', 'source.zip'):
        files['TinyCC/' + name] = ROOT / 'TinyCC' / name
    for path in (ROOT / 'Licenses').rglob('*'):
        if path.is_file():
            files[path.relative_to(ROOT).as_posix()] = path
    for name in ('portable', 'fps_overlay', 'menu_shortcuts', 'save_states', 'save_backups'):
        files['docs/' + name + '.md'] = ROOT / 'docs' / (name + '.md')
    for name, source in files.items():
        if source.is_symlink() or not source.is_file():
            raise SystemExit(f'Missing regular package input: {source} ({name})')
    args.output.parent.mkdir(parents=True, exist_ok=True)
    # Build outside the final archive: failed packaging keeps an earlier ZIP intact.
    with tempfile.TemporaryDirectory(dir=args.output.parent) as staging:
        archive = Path(staging) / 'portable.zip'
        with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as bundle:
            prefix = 'FFTARecomp-portable/'
            for name, source in sorted(files.items()):
                bundle.write(source, prefix + name)
            bundle.writestr(prefix + 'portable.txt', 'FFTARecomp portable mode. Keep this file beside both executables.\n')
            bundle.writestr(prefix + 'LEGGIMI.txt', '''FFTARecomp portable - Windows x86_64
Estrai tutta la cartella in una posizione scrivibile e avvia FFTARecomp.exe.
Al primo avvio scegli la tua ROM FFTA USA/Europa: viene copiata come ffta.gba.
Puoi anche copiarla manualmente accanto agli eseguibili prima di avviare.
Nessuna ROM o BIOS e incluso nel pacchetto. Nessuna installazione richiesta.

Tutti i dati dell'app restano nella cartella Data. Per spostare il gioco,
copia TUTTA la cartella, inclusi Data, ffta.gba e portable.txt, a gioco chiuso.
Per importare progressi: a gioco chiuso copia il tuo .sav in Data/saves/ffta.sav.
Se esiste gia, conservane prima una copia. Dettagli: docs/portable.md.
Menu: Esc oppure Start+Back/Select sul controller; combo configurabili in Input.
Savestate: menu Save states (9 slot con anteprima e annulla).
Backup del .sav: menu Save backups (le ultime 2 copie create all'avvio).
Per errori di avvio consulta play-error.log nella cartella principale.
''')
            for directory in ('config', 'cache', 'saves'):
                bundle.writestr(prefix + 'Data/' + directory + '/', b'')
        with zipfile.ZipFile(archive) as bundle:
            if bundle.testzip() is not None:
                raise SystemExit('Archive CRC validation failed')
            for name, source in files.items():
                if bundle.read(prefix + name) != source.read_bytes():
                    raise SystemExit(f'Archive input mismatch: {name}')
        archive.replace(args.output)
    print(f'Created {args.output} ({args.output.stat().st_size:,} bytes)')


if __name__ == '__main__':
    main()
