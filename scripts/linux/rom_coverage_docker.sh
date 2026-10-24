#!/usr/bin/env bash
# Generate the atlas with Python, Cargo and nm inside Docker.
set -euo pipefail
repo_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
rom_path='rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba'
dataset_dir='target/rom-data-rank-gates/windows-managed-europe/offline dataset'
translation_path='out/1194 - Final Fantasy Tactics Advance (E)(Surplus).dll'
output_dir='docs/development/rom-coverage'
evidence_path='docs/development/rom-coverage/evidence.json'
usage() {
    printf 'Usage: %s [--rom FILE] [--dataset DIR] [--translation FILE] [--output DIR] [--evidence FILE]\n' "$0"
    printf 'Defaults use the frozen EUR e37-d36 inputs; all relative paths start at the project root.\n'
}
while (( $# )); do
    case "$1" in
        --help|-h) usage; exit 0 ;;
        --rom|--dataset|--translation|--output|--evidence)
            if (( $# < 2 )) || [[ -z "$2" ]]; then
                printf 'Missing value for %s\n' "$1" >&2; exit 2
            fi
            case "$1" in
                --rom) rom_path=$2 ;;
                --dataset) dataset_dir=$2 ;;
                --translation) translation_path=$2 ;;
                --output) output_dir=$2 ;;
                --evidence) evidence_path=$2 ;;
            esac
            shift 2 ;;
        *) printf 'Unknown argument: %s\n' "$1" >&2; usage >&2; exit 2 ;;
    esac
done
resolve_path() {
    local value=$1
    if [[ "$value" != /* ]]; then value="$repo_dir/$value"; fi
    # Docker --mount parses CSV, including double quotes.
    if [[ "$value" == *','* || "$value" == *'"'* || "$value" == *$'\n'* || "$value" == *$'\r'* ]]; then
        printf 'Paths must not contain commas, double quotes or newlines.\n' >&2; return 2
    fi
    printf '%s' "$value"
}
rom_path=$(resolve_path "$rom_path")
dataset_dir=$(resolve_path "$dataset_dir")
translation_path=$(resolve_path "$translation_path")
output_dir=$(resolve_path "$output_dir")
evidence_path=$(resolve_path "$evidence_path")
for path in "$rom_path" "$translation_path" "$evidence_path" "$dataset_dir/manifest.toml"; do
    if [[ ! -f "$path" ]]; then printf 'Required input file does not exist: %s\n' "$path" >&2; exit 2; fi
done
command -v docker >/dev/null || { printf 'Docker with Buildx is required.\n' >&2; exit 1; }
mkdir -p -- "$output_dir"
# Canonical paths also catch CSV-unsafe characters introduced by symlinks.
rom_path=$(resolve_path "$(cd -- "$(dirname -- "$rom_path")" && pwd -P)/$(basename -- "$rom_path")")
translation_path=$(resolve_path "$(cd -- "$(dirname -- "$translation_path")" && pwd -P)/$(basename -- "$translation_path")")
evidence_path=$(resolve_path "$(cd -- "$(dirname -- "$evidence_path")" && pwd -P)/$(basename -- "$evidence_path")")
dataset_dir=$(resolve_path "$(cd -- "$dataset_dir" && pwd -P)")
output_dir=$(resolve_path "$(cd -- "$output_dir" && pwd -P)")
image=ffta-rom-coverage:local
docker buildx build --platform linux/amd64 \
    --file "$repo_dir/Dockerfile.linux" --target coverage \
    --load --tag "$image" --progress plain "$repo_dir"
docker run --rm --platform linux/amd64 --network none \
    --user "$(id -u):$(id -g)" --env HOME=/tmp \
    --mount "type=bind,source=$rom_path,target=/input/source.gba,readonly" \
    --mount "type=bind,source=$dataset_dir,target=/input/dataset,readonly" \
    --mount "type=bind,source=$translation_path,target=/input/translation.dll,readonly" \
    --mount "type=bind,source=$evidence_path,target=/input/evidence.json,readonly" \
    --mount "type=bind,source=$output_dir,target=/output" \
    "$image" --rom /input/source.gba --dataset /input/dataset \
    --translation /input/translation.dll --evidence /input/evidence.json --output /output
printf '\nROM coverage on host: %s/index.html\n' "$output_dir"
