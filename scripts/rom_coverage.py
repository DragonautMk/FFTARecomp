#!/usr/bin/env python3
"""Build a metadata-only ROM coverage atlas. Python 3.11+, standard library.

Inputs are read-only. No ROM bytes, instructions, text or decoded assets enter
the report. Static candidate code, runtime availability and ordinary positive
resource use are deliberately separate measurements.
"""
import argparse
import collections
import hashlib
import json
import pathlib
import re
import sqlite3
import subprocess
import tempfile
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def sha(path):
    with open(path, "rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def union(ranges):
    result = []
    for start, end in sorted(ranges):
        if start < 0 or end <= start:
            raise ValueError(f"invalid interval {start}..{end}")
        if result and start <= result[-1][1]:
            result[-1][1] = max(end, result[-1][1])
        else:
            result.append([start, end])
    return result


def length(ranges):
    return sum(b - a for a, b in union(ranges))


def intersection(left, right):
    left, right = union(left), union(right)
    i = j = 0
    result = []
    while i < len(left) and j < len(right):
        a, b = max(left[i][0], right[j][0]), min(left[i][1], right[j][1])
        if a < b:
            result.append([a, b])
        if left[i][1] < right[j][1]:
            i += 1
        else:
            j += 1
    return union(result)


def complement(ranges, size):
    cursor, result = 0, []
    for a, b in union(ranges):
        if b > size:
            raise ValueError("range outside ROM")
        if cursor < a:
            result.append([cursor, a])
        cursor = b
    if cursor < size:
        result.append([cursor, size])
    return result


def multiple_owners(owners):
    """Intervals covered by >=2 distinct owners; duplicate roles count once."""
    events = collections.defaultdict(list)
    for owner, spans in owners.items():
        for a, b in union(spans):
            events[a].append((owner, 1))
            events[b].append((owner, -1))
    active, result, previous = set(), [], 0
    for offset in sorted(events):
        if len(active) >= 2 and previous < offset:
            result.append([previous, offset])
        for owner, direction in events[offset]:
            if direction == 1:
                active.add(owner)
            else:
                active.remove(owner)
        previous = offset
    return union(result)


def checked_span(start, end, size):
    if not (isinstance(start, int) and isinstance(end, int) and 0 <= start < end <= size):
        raise ValueError(f"out-of-ROM interval {start}..{end}")
    return [start, end]


def code_ranges(rom_path, translation, analyzer, rom, work):
    symbols = subprocess.check_output(["nm", "--defined-only", str(translation)], text=True)
    entries = {(int(addr, 16) | (mode == "t")) for addr, mode in
               re.findall(r"\bb_([0-9a-fA-F]{8})_([ta])\b", symbols)}
    seeds = sorted(key for key in entries if 0x08000000 <= (key & ~1) < 0x08000000 + len(rom))
    if not seeds:
        raise ValueError("no native ROM entry symbols in translation")
    seed_file = work / "seeds.txt"
    seed_file.write_text("".join(f"{key:08x}\n" for key in seeds))
    lines = subprocess.check_output([str(analyzer.resolve()), str(rom_path.resolve()),
                                     str(seed_file)], text=True).splitlines()
    fnv = 0xcbf29ce484222325
    for byte in rom:
        # Match recomp_core::fnv64, including its existing multiplier.
        fnv = ((fnv ^ byte) * 0x1_0000_01b3) & 0xffffffffffffffff
    expected = f"# rom_bytes={len(rom)} rom_fnv64={fnv:016x} seeds={len(seeds)}"
    if lines[0] != expected or lines[1] != "start\tend\tmode\tinstructions\tretained_entry":
        raise ValueError("analyzer/ROM identity mismatch")
    spans, by_mode, retained, instruction_visits = [], collections.defaultdict(list), 0, 0
    for line in lines[2:]:
        a, b, mode, count, old = line.split("\t")
        if mode not in ("arm", "thumb") or old not in ("0", "1"):
            raise ValueError("invalid analyzer row")
        span = checked_span(int(a), int(b), len(rom))
        if int(count) * (2 if mode == "thumb" else 4) != span[1] - span[0]:
            raise ValueError("instruction span/width mismatch")
        spans.append(span)
        by_mode[mode].append(span)
        retained += int(old)
        instruction_visits += int(count)
    if retained != len(seeds):
        raise ValueError("retained entry missing from analysis")
    return union(spans), {
        "method": "Current recursive analyzer seeded by retained native ROM block symbols; speculative candidates, not executed-byte coverage or semantic function coverage.",
        "retained_rom_entries": len(seeds), "excluded_non_rom_entries": len(entries) - len(seeds),
        "discovered_blocks": len(spans), "instruction_visits_with_overlap": instruction_visits,
        "arm_bytes": length(by_mode["arm"]), "thumb_bytes": length(by_mode["thumb"]),
        "arm_thumb_overlap_bytes": length(intersection(by_mode["arm"], by_mode["thumb"])),
        "translation_sha256": sha(translation), "analyzer_executable_sha256": sha(analyzer),
        "analyzer_source_sha256": sha(ROOT / "crates/recomp-core/src/analyze.rs"),
        "seeds_sha256": sha(seed_file),
    }


def collect_sharing(dataset, manifest):
    """Explicit same-address references, separate from cross-resource ownership.

    Bounded lower bound: BG descriptors, packet indexes, transfer links, and
    same-resource manifest aliases. Identical payloads at different offsets are
    never classified as shared addresses.
    """
    resources = {r["id"]: r for r in manifest["resources"]}
    refs = collections.defaultdict(set)
    spans = {}
    starts = {}
    for rid, r in resources.items():
        by_range = collections.defaultdict(list)
        for source in r["sources"]:
            a, b = source["offset"], source["offset"] + source["length"]
            starts[rid, a] = (a, b)
            by_range[a, b].append(source["role"])
        for span, roles in by_range.items():
            if len(set(roles)) >= 2:
                key = rid, *span
                spans[key] = list(span)
                refs[key].update("manifest:" + role for role in roles)

    def link(rid, start, ref):
        if (rid, start) not in starts:
            raise ValueError(f"unresolved sharing link {rid} at {start:#x}")
        a, b = starts[rid, start]
        key = rid, a, b
        spans[key] = [a, b]
        refs[key].add(ref)

    def load(rid):
        return json.loads((dataset / resources[rid]["path"]).read_text())

    root = resources["graphics.bg-descriptors"]["sources"][0]["offset"]
    for i, desc in enumerate(load("graphics.bg-descriptors")["entries"]):
        for field, rid in (("graphics_source", "graphics.bg-main-graphics"),
                           ("palette_source", "graphics.bg-base-palettes")):
            link(rid, root + desc[field], f"descriptor/{i}/{field}")
    for plane, catalog in load("graphics.bg-packet-programs").items():
        for i, index in enumerate(catalog["descriptor_programs"]):
            if index is not None:
                program = catalog["programs"][index]
                link("graphics.bg-packet-programs", root + program["relative_source"],
                     f"descriptor/{i}/{plane}")
    transfers = load("graphics.bg-transfer-programs")
    for i, row in enumerate(transfers["descriptor_links"]):
        for j, entry in enumerate(row):
            if entry is not None:
                for field, records in (("control", "controls"), ("tiles", "tiles")):
                    source = transfers[records][entry[field]]["relative_source"]
                    link("graphics.bg-transfer-programs", root + source, f"descriptor/{i}/transfer/{j}/{field}")
    result = []
    for key in sorted(refs):
        if len(refs[key]) >= 2:
            # Prefer explicit decoded links to duplicate manifest role labels.
            explicit = [r for r in refs[key] if not r.startswith("manifest:")]
            selected = explicit if len(explicit) >= 2 else sorted(refs[key])
            if len(selected) >= 2:
                result.append({"resource": key[0], "range": spans[key],
                               "references": len(selected), "examples": sorted(selected)[:4]})
    return result


def build(args):
    rom_path, dataset = pathlib.Path(args.rom), pathlib.Path(args.dataset)
    rom = rom_path.read_bytes()
    manifest_path = dataset / "manifest.toml"
    manifest = tomllib.loads(manifest_path.read_text())
    evidence = json.loads(pathlib.Path(args.evidence).read_text())
    digest = hashlib.sha256(rom).hexdigest()
    if digest != manifest["source_rom_sha256"] or len(rom) != manifest["source_rom_size"]:
        raise ValueError("manifest/ROM identity mismatch")
    if [digest, manifest["extractor_revision"], manifest["decoder_revision"]] != evidence["baseline"]:
        raise ValueError("evidence is for a different baseline; review it before regeneration")
    if sha(args.translation) != evidence["translation_sha256"]:
        raise ValueError("translation differs from the retained evidence baseline")
    records, owners, all_data, ordinary = [], {}, [], []
    for resource in manifest["resources"]:
        rid = resource["id"]
        path = (dataset / resource["path"]).resolve()
        if not path.is_relative_to(dataset.resolve()) or sha(path) != resource["sha256"]:
            raise ValueError(f"invalid decoded resource {rid}")
        spans = []
        for s in resource["sources"]:
            a, b = checked_span(s["offset"], s["offset"] + s["length"], len(rom))
            if hashlib.sha256(rom[a:b]).hexdigest() != s["sha256"]:
                raise ValueError(f"source authentication failed: {rid}")
            spans.append([a, b])
        note = evidence["resources"][rid]
        if not note["references"] or any(not (ROOT / ref).is_file() for ref in note["references"]):
            raise ValueError(f"missing evidence record {rid}")
        spans = union(spans)
        owners[rid] = spans
        all_data.extend(spans)
        if note["ordinary_positive"]:
            ordinary.extend(spans)
        records.append({"id": rid, "ranges": spans, "source_bytes": length(spans),
                        "manifest_roles": len(resource["sources"]), "runtime_available": True,
                        "decoded_sha256": resource["sha256"], **note})
    runtime = union(all_data)
    for extra in evidence["source_only"]:
        spans = [checked_span(a, b, len(rom)) for a, b in extra["ranges"]]
        if any(not (ROOT / ref).is_file() for ref in extra["references"]):
            raise ValueError("missing source-only evidence")
        all_data.extend(spans)
        records.append({**extra, "ranges": spans, "source_bytes": length(spans),
                        "runtime_available": False, "ordinary_positive": False})
    data = union(all_data)
    with tempfile.TemporaryDirectory() as work:
        code, code_info = code_ranges(rom_path, pathlib.Path(args.translation),
                                      pathlib.Path(args.analyzer), rom, pathlib.Path(work))
    db_path = ROOT / "gamedb.sqlite"
    with sqlite3.connect(db_path.as_uri() + "?mode=ro", uri=True) as db:
        code_info["gamedb_functions_for_rom"] = db.execute(
            "SELECT COUNT(*) FROM functions WHERE sha256 = ?", (digest,)).fetchone()[0]
    code_info["gamedb_sha256"] = sha(db_path)
    shared_records = collect_sharing(dataset, manifest)
    shared = union([r["range"] for r in shared_records])
    owner_overlap = multiple_owners(owners)
    header = [[4, 192]]  # Header branch 0..4 is code, not cartridge metadata.
    unknown = complement(code + data + header, len(rom))
    layers = {"code_candidates": code, "identified_data": data,
              "runtime_available": runtime, "ordinary_resource_extents": union(ordinary),
              "shared_references": shared, "multiple_resource_owners": owner_overlap,
              "header_metadata": header, "unclassified": unknown,
              "code_data_overlap": intersection(code, data)}
    totals = {name: length(spans) for name, spans in layers.items()}
    totals.update({"rom": len(rom), "identified_families": len(records),
                   "runtime_families": len(manifest["resources"]),
                   "ordinary_positive_families": sum(r["ordinary_positive"] for r in records),
                   "ordinary_distinct_read_bytes": None,
                   "identified_union_with_header": length(data + header),
                   "shared_records": len(shared_records)})
    return {"schema": 1, "date": evidence["date"], "profile": manifest["profile"],
            "rom_sha256": digest, "generation": f"e{manifest['extractor_revision']}-d{manifest['decoder_revision']}",
            "manifest_sha256": sha(manifest_path), "evidence_sha256": sha(args.evidence),
            "code": code_info, "totals": totals, "layers": layers,
            "resources": records, "sharing": shared_records,
            "limits": evidence["limits"], "bin_bytes": 65536,
            "largest_unclassified": sorted(unknown, key=lambda r: r[1] - r[0], reverse=True)[:20]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rom", required=True)
    parser.add_argument("--dataset", required=True)
    parser.add_argument("--translation", required=True, help="retained native ELF with block symbols; never rebuilt")
    parser.add_argument("--analyzer", required=True, help="rom_code_ranges executable")
    parser.add_argument("--evidence", default=str(ROOT / "docs/development/rom-coverage/evidence.json"))
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    report = build(args)
    output = pathlib.Path(args.output)
    output.mkdir(parents=True, exist_ok=True)
    payload = json.dumps(report, ensure_ascii=False, separators=(",", ":"))
    (output / "coverage.json").write_text(payload + "\n")
    template = pathlib.Path(__file__).with_name("rom_coverage.html").read_text()
    (output / "index.html").write_text(template.replace("__ROM_COVERAGE_DATA__", payload.replace("<", "\\u003c")))
    print(json.dumps(report["totals"], indent=2))


if __name__ == "__main__":
    main()
