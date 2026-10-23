#!/usr/bin/env python3
"""Extract readable ROM data through the project's source-authenticated CLI."""

import argparse
from pathlib import Path
import shutil
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
DESCRIPTIONS = {
    'linked-metadata4.json': 'European paired selectors/values; original live qualification scans retained',
    'linked-metadata2.json': 'European stored key/value map; distinct original forward and inverse extents',
    'default-constructor-template.json': 'European default constructor template; original live patches and alternate sources retained',
    'constructor-alternate-templates.json': 'European four alternate constructor records and parsed base pointer; live patches retained',
    'linked-metadata14.json': 'European special-unit fields and packed palette selectors; live books and overrides retained',
    'linked-metadata12.json': 'European indexed packed two-bit fields; original row and mode arithmetic retained',
    'global-pair-flags.json': 'European global packed pair flags; original live mask and early exit retained',
    'linked-metadata52.json': 'European packed metadata and redirects; original live alternate indices retained',
    'linked-metadata16.json': 'European sixteen-byte linked metadata; original live list and RNG retained',
    'item-links.json': 'European counted item-linked pairs; original live unit publication retained',
    'items.json': 'European static item fields; original live property modifiers retained',
    'manifest.toml': 'source identity, resource coverage and hashes',
    'ui-graphics.json': 'European custom UI graphics word banks for BG/OBJ uploads',
    'ui-palettes.json': 'European raw UI palette words for BG/OBJ shadows',
    'obj-graphics.json': 'European custom OBJ word banks and language selector tables',
    'obj-animation-banks.json': 'European indexed sprite lists, animation frames and durations',
    'obj-static-lists.json': 'European static sprite lists with coordinates, attributes, tiles and palette indices',
    'obj-dimensions.toml': 'sprite dimensions',
    'bg-descriptors.json': 'background descriptors and source links',
    'bg-base-palettes.json': 'base background palette banks',
    'bg-alternate-palettes.json': 'alternate background palette banks',
    'bg-packet-programs.json': 'background packet programs and ordered writes',
    'bg-main-graphics.json': 'European background graphics words and destinations',
    'bg-transfer-programs.json': 'European background transfer controls and shared tile banks',
    'short-texts.json': 'European short strings, game glyph codes, font slots and widths',
    'text-catalogs.json': 'European pointer strings, relative text archives and referenced font slots',
    'font-bank.json': 'European complete packed font bank and glyph widths',
    'name-keyboard.json': 'European name-entry programs, navigation widths and selectable cells',
}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('rom', type=Path, help='path to your supported FFTA ROM')
    parser.add_argument('--output', type=Path,
                        help='dataset directory (default: target/rom-data/<ROM filename stem>)')
    parser.add_argument('--recomp', type=Path,
                        help='existing recomp executable; otherwise Cargo builds/runs release')
    parser.add_argument('--wine', action='store_true',
                        help='run the executable supplied with --recomp through Wine')
    args = parser.parse_args(argv)
    rom = args.rom.expanduser().resolve()
    if not rom.is_file():
        parser.error(f'ROM file does not exist: {rom}')
    if args.wine and args.recomp is None:
        parser.error('--wine requires --recomp pointing to a Windows executable')
    output = (args.output.expanduser() if args.output is not None
              else ROOT / 'target' / 'rom-data' / rom.stem).resolve()

    if args.recomp is not None:
        executable = args.recomp.expanduser().resolve()
        if not executable.is_file():
            parser.error(f'recomp executable does not exist: {executable}')
        command = [str(executable)]
        if args.wine:
            wine = shutil.which('wine')
            if wine is None:
                parser.error('Wine was not found in PATH')
            command.insert(0, wine)
    else:
        cargo = shutil.which('cargo')
        if cargo is None:
            parser.error('Cargo was not found in PATH; install Rust or use --recomp')
        command = [cargo, 'run', '--locked', '--release', '--package', 'recomp',
                   '--bin', 'recomp', '--']
    command += ['rom-data', 'extract', str(rom), str(output)]

    print(f'Extracting ROM data into: {output}', flush=True)
    try:
        result = subprocess.run(command, cwd=ROOT)
    except OSError as error:
        print(f'Cannot run extraction: {error}', file=sys.stderr)
        return 1
    if result.returncode:
        print('Extraction failed; see the CLI error above.', file=sys.stderr)
        return result.returncode if result.returncode > 0 else 1

    try:
        if not (output / 'manifest.toml').is_file():
            raise OSError('extraction returned success without a manifest.toml')
        files = sorted(path for path in output.iterdir()
                       if path.is_file() and path.suffix in ('.json', '.toml'))
    except OSError as error:
        print(f'Cannot inspect extracted dataset: {error}', file=sys.stderr)
        return 1
    print(f'\nDataset ready: {output}\n{len(files)} readable files:')
    for path in files:
        description = DESCRIPTIONS.get(path.name, 'decoded resource')
        print(f'  {path.name} - {description}')
    print('\nOpen these JSON/TOML files in your editor to inspect the extracted data.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
