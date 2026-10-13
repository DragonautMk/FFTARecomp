"""Read an existing FFTA log; emit observations only under FFTA_OAM_LOG=1."""
import argparse
import bisect
import collections
import json
import os
import re
from pathlib import Path


def analyze(path):
    raw = Path(path).read_bytes()
    text = raw.decode("utf-16" if raw[:2] in (b"\xff\xfe", b"\xfe\xff") else "utf-8-sig", errors="replace")
    bg_pattern = r"FFTA_BG frame=(\d+) bg=(\d+) control=0x([0-9A-F]+) hofs=(\d+) vofs=(\d+) dispcnt=0x([0-9A-F]+)"
    dma_pattern = r"FFTA_DMA frame=(\d+) channel=(\d+) src=0x([0-9A-F]+) dst=0x([0-9A-F]+) count=(\d+) unit=(\d+) timing=(\d+)\s+control=0x([0-9A-F]+)"
    bg = [tuple(int(v, 16 if i in (2, 5) else 10) for i, v in enumerate(m)) for m in re.findall(bg_pattern, text)]
    dma = [tuple(int(v, 16 if i in (2, 3, 7) else 10) for i, v in enumerate(m)) for m in re.findall(dma_pattern, text)]
    active = collections.defaultdict(list)
    terrain = {}
    for frame, index, cnt, h, v, disp in bg:
        if not (disp & (1 << (8 + index))) or not (disp & 7 == 0 or disp & 7 == 1 and index < 2):
            continue
        active[index, cnt].append((frame, h, v))
        if cnt in (0x4C03, 0x4E02):
            terrain[frame] = (h, v)
    backgrounds = []
    for (index, cnt), samples in sorted(active.items()):
        width, height = [(256, 256), (512, 256), (256, 512), (512, 512)][cnt >> 14]
        backgrounds.append(dict(status="MEDIDO", bg=index, cnt=f"0x{cnt:04X}", width_px=width, height_px=height,
            char_base=f"0x{0x06000000 + ((cnt >> 2) & 3) * 0x4000:08X}",
            screen_base=f"0x{0x06000000 + ((cnt >> 8) & 31) * 0x800:08X}",
            hofs=[min(s[1] for s in samples), max(s[1] for s in samples)],
            vofs=[min(s[2] for s in samples), max(s[2] for s in samples)],
            first_frame=samples[0][0], last_frame=samples[-1][0], samples=len(samples)))
    frames = sorted(terrain)
    groups = collections.defaultdict(list)
    destinations = collections.defaultdict(set)
    for frame, channel, src, dst, count, unit, timing, control in dma:
        if not 0x06000000 <= dst < 0x06018000:
            continue
        i = bisect.bisect_right(frames, frame) - 1
        camera = terrain[frames[i]] if i >= 0 and frame - frames[i] < 30 else None
        groups[dst, src, count, unit].append((frame, channel, timing, camera))
        destinations[dst].add((src, count, unit))
    transfers = []
    for (dst, src, count, unit), events in sorted(groups.items()):
        cameras = sorted({e[3] for e in events if e[3] is not None})
        transfers.append(dict(status="MEDIDO", destination=f"0x{dst:08X}", source=f"0x{src:08X}",
            region="BG" if dst < 0x06010000 else "OBJ", count=count, unit=unit, events=len(events),
            first_frame=min(e[0] for e in events), last_frame=max(e[0] for e in events),
            channels=sorted({e[1] for e in events}), timings=sorted({e[2] for e in events}),
            preceding_camera_samples=cameras, camera_association_status="SUPOSICION: preceding sample, at most 29 frames old; no causal attribution",
            caller_status="NO MEDIDO: original DMA records have no PC/LR"))
    bounds = None
    if terrain:
        hmin, hmax = min(s[0] for s in terrain.values()), max(s[0] for s in terrain.values())
        vmin, vmax = min(s[1] for s in terrain.values()), max(s[1] for s in terrain.values())
        bounds = dict(status="MEDIDO", terrain_identification="SUPOSICION: scrolling pair CNT 4C03/4E02",
            hofs=[hmin, hmax], vofs=[vmin, vmax], register_span_px=[hmax-hmin, vmax-vmin],
            central_x_at_extremes=[[hmin,hmin+239],[hmax,hmax+239]],
            central_y_at_extremes=[[vmin,vmin+159],[vmax,vmax+159]],
            wide_x_32_9_at_extremes=[[hmin-164,hmin+403],[hmax-164,hmax+403]],
            world_camera_displacement_status="SUPOSICION: register span equals world displacement only without remapping/wrapping",
            map_content_status="NO MEDIDO: no tilemap snapshots in capture 1")
    return dict(status="MEDIDO: parsed existing capture; no emulation or new capture", input=str(path),
        bg_samples=len(bg), dma_events=len(dma), backgrounds=backgrounds, terrain_bounds=bounds,
        dma_bg_events=sum(t["events"] for t in transfers if t["region"]=="BG"),
        dma_obj_events=sum(t["events"] for t in transfers if t["region"]=="OBJ"),
        dma_destinations=[dict(destination=f"0x{dst:08X}", variants=len(v)) for dst,v in sorted(destinations.items())],
        dma_groups=transfers,
        missing=dict(status="NO MEDIDO", data=["camera RAM variables", "world map width/height in tiles",
            "offscreen tilemap contents", "sprite rejection counters", "DMA caller PC/LR",
            "instructions and callers producing 0x0200F3DE", "CPU writes to OBJ VRAM"],
            available="Hardware tilemap size is derived from BGxCNT; it is not the world map size."))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("log")
    parser.add_argument("--output")
    args = parser.parse_args()
    if os.environ.get("FFTA_OAM_LOG") != "1":
        parser.error("FFTA_OAM_LOG=1 is required")
    report = analyze(args.log)
    if args.output:
        target = Path(args.output)
        # Preserve prior artifacts rather than silently replacing them.
        with target.open("x", encoding="utf-8") as out:
            json.dump(report, out, indent=2)
        print(f"MEDIDO report={target} bg_samples={report['bg_samples']} DMA_BG={report['dma_bg_events']} DMA_OBJ={report['dma_obj_events']}")
    else:
        print(json.dumps(report, indent=2))
