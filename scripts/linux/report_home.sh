#!/usr/bin/env bash
# Reuse the existing coverage image; keep the checkout readonly inside Docker.
set -euo pipefail
repo_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
if (( $# > 0 )); then
    printf 'Usage: bash scripts/linux/report_home.sh\nRequires existing ffta-rom-coverage:local Docker image; no build or extraction.\n' >&2
    exit 2
fi
if [[ "$repo_dir" == *','* || "$repo_dir" == *'"'* || "$repo_dir" == *$'\n'* || "$repo_dir" == *$'\r'* ]]; then
    printf 'Docker mount paths cannot contain commas, double quotes or newlines.\n' >&2
    exit 2
fi
command -v docker >/dev/null || { printf 'Docker required. Alternative: python3 scripts/report_home.py\n' >&2; exit 1; }
docker image inspect ffta-rom-coverage:local >/dev/null || {
    printf 'Cannot access coverage image. Check Docker daemon/image, or use python3 scripts/report_home.py. No build started.\n' >&2
    exit 1
}
temporary=$(mktemp "$repo_dir/.report-home-XXXXXXXX.tmp")
trap 'rm -f -- "$temporary"' EXIT
docker run --rm --network none --entrypoint python3 \
    --mount "type=bind,source=$repo_dir,target=/project,readonly" \
    ffta-rom-coverage:local /project/scripts/report_home.py --root /project --stdout > "$temporary"
mv -f -- "$temporary" "$repo_dir/index.html"
printf 'Homepage: %s/index.html\n' "$repo_dir"
