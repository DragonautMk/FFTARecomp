#!/usr/bin/env python3
"""Retire disposable validation copies, retaining evidence and accepted datasets.

`complete` is an explicit assertion that a unit's managed/native checks ended.
Preview is the default. Paths containing links, private files, unexpected files
or tracked content are refused. No Docker/global cache or Cargo cleanup occurs.
"""
import argparse
import datetime
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
SCRATCH_GROUPS = ("interrupted", "moved resources")
EXECUTABLE = re.compile(r"(?:recomp(?:-tests[^/]*|-rom-data)?|gba-core-tests[^/]*)$")
PRIVATE = {".gba", ".sav", ".ffstate", ".input"}


def inventory(path, *, binary=False):
    """Do not follow even internal links: test payloads may point outside."""
    files, dirs = [], []
    if path.is_symlink():
        raise ValueError(f"symlink refused: {path}")
    if path.is_dir():
        for parent, children, names in os.walk(path, followlinks=False):
            for name in children:
                p = Path(parent) / name
                if p.is_symlink():
                    raise ValueError(f"symlink refused: {p}")
                dirs.append(p)
            files.extend(Path(parent) / name for name in names)
    elif path.is_file():
        files.append(path)
    else:
        return [], []
    result = []
    for p in files:
        if p.is_symlink() or not p.is_file():
            raise ValueError(f"non-regular file refused: {p}")
        if p.suffix.lower() in PRIVATE:
            raise ValueError(f"private input refused: {p}")
        if not binary and p.suffix not in (".json", ".toml", ".lock") and p.name != "current":
            raise ValueError(f"unexpected fixture file refused: {p}")
        st = p.stat()
        result.append({"path": str(p), "size": st.st_size, "mtime_ns": st.st_mtime_ns,
                       "allocated": getattr(st, "st_blocks", 0) * 512,
                       "identity": [st.st_dev, st.st_ino]})
    return sorted(result, key=lambda r: r["path"]), sorted(map(str, dirs))


def ensure_untracked(path, root):
    tracked = subprocess.check_output(["git", "-C", str(root), "ls-files", "-z", "--",
                                       str(path.relative_to(root))])
    if tracked:
        raise ValueError(f"tracked content refused: {path}")


def remove_candidate(item):
    path = Path(item["path"])
    files, dirs = inventory(path, binary=item["kind"] == "executable")
    if files != item["files"] or dirs != item["directories"]:
        raise ValueError(f"candidate changed since preview: {path}")
    if path.is_dir():
        shutil.rmtree(path)
    else:
        path.unlink()


def completion_plan(units, root=ROOT, retire_executables=False):
    root = root.resolve()
    target = root / "target"
    if target.is_symlink():
        raise ValueError("target must not be a symlink")
    candidates, skipped = [], []
    for value in units:
        unit = Path(os.path.abspath(value))
        if unit.parent != target or not unit.name.startswith("rom-data-") or unit.is_symlink():
            raise ValueError(f"expected a real target/rom-data-* unit directory: {unit}")
        if not unit.is_dir():
            raise ValueError(f"unit does not exist: {unit}")
        if not any("result" in p.name or "summary" in p.name for p in unit.glob("*.json")):
            raise ValueError(f"unit has no result/summary evidence: {unit}")
        managed = unit / "windows-managed-europe"
        paths = [] if managed.is_symlink() else [(managed / n, "fixture") for n in SCRATCH_GROUPS]
        if retire_executables:
            paths += [(p, "executable") for p in unit.iterdir()
                      if p.is_file() and p.suffix == ".exe" and EXECUTABLE.fullmatch(p.stem)]
        for path, kind in paths:
            if not path.exists() and not path.is_symlink():
                continue
            try:
                ensure_untracked(path, root)
                files, dirs = inventory(path, binary=kind == "executable")
                candidates.append({"path": str(path), "kind": kind, "files": files,
                                   "directories": dirs, "bytes": sum(f["size"] for f in files),
                                   "allocated": sum(f["allocated"] for f in files)})
            except ValueError as error:
                skipped.append(str(error))
    return candidates, skipped


def prune_coverage_cache(cache, current, keep):
    """Bound only authenticated-layout coverage caches after a successful run.

    Explicit datasets never call this. Callers retaining a read-only cache use
    keep=0. The current dataset is retained even if a newer cache also exists.
    """
    if keep < 1:
        return []
    cache, current = Path(cache).absolute(), Path(current).absolute()
    if (cache.is_symlink() or current.is_symlink() or current.parent.parent != cache
            or current.parent.is_symlink()):
        raise ValueError("unexpected coverage cache layout")
    if not re.fullmatch(r"[0-9a-f]{64}", current.parent.name):
        raise ValueError("unexpected coverage ROM directory")
    import tomllib  # Coverage already requires Python 3.11; completion does not.
    datasets = []
    for path in current.parent.iterdir():
        match = re.fullmatch(r"e(\d+)-d(\d+)", path.name)
        if not match or path.is_symlink() or not path.is_dir():
            continue
        manifest = path / "manifest.toml"
        if manifest.is_symlink() or not manifest.is_file():
            continue
        data = tomllib.loads(manifest.read_text())
        revision = tuple(map(int, match.groups()))
        if data.get("source_rom_sha256") != current.parent.name or revision != (
                data.get("extractor_revision"), data.get("decoder_revision")):
            continue
        datasets.append((revision, path))
    survivors = {current}
    for _, path in sorted(datasets, reverse=True):
        if len(survivors) >= keep:
            break
        survivors.add(path)
    removed = []
    for _, path in datasets:
        if path in survivors:
            continue
        files, dirs = inventory(path)
        remove_candidate(dict(path=str(path), files=files, directories=dirs, kind="fixture"))
        removed.append(str(path))
    return removed


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["complete"])
    parser.add_argument("units", nargs="+", type=Path)
    parser.add_argument("--apply", action="store_true", help="remove the reviewed disposable copies")
    parser.add_argument("--retire-executables", action="store_true",
                        help="also retire old unit-local test binaries; retain shared/canonical binaries")
    parser.add_argument("--report", type=Path, help="write a JSON cleanup manifest under target")
    args = parser.parse_args()
    try:
        candidates, skipped = completion_plan(args.units, root=ROOT,
                                               retire_executables=args.retire_executables)
        report = {"date": datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  "applied": args.apply, "candidates": candidates, "skipped": skipped,
                  "bytes": sum(c["bytes"] for c in candidates),
                  "allocated": sum(c["allocated"] for c in candidates), "removed": []}
        report_path = args.report.resolve() if args.report else None
        if report_path and (not report_path.is_relative_to(ROOT / "target") or
                            any(report_path == Path(c["path"]) or report_path.is_relative_to(c["path"])
                                for c in candidates)):
            raise ValueError("report must be under target, outside removal candidates")
        if report_path:
            report_path.parent.mkdir(parents=True, exist_ok=True)
            report_path.write_text(json.dumps(report, indent=2) + "\n")
        if args.apply:
            for item in candidates:
                ensure_untracked(Path(item["path"]), ROOT)
                remove_candidate(item)
                report["removed"].append(item["path"])
                if report_path:
                    report_path.write_text(json.dumps(report, indent=2) + "\n")
        print(f"{'Removed' if args.apply else 'Preview'}: {len(candidates)} paths, "
              f"{report['allocated'] / 10**9:.2f} GB allocated; {len(skipped)} skipped")
        for reason in skipped:
            print(reason)
        if report_path:
            print(f"Manifest: {report_path}")
        return 0
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(f"Artifact cleanup failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
