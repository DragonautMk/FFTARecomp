"""Convert captured P6 RGB frames to PNG using only the Python standard library."""
import argparse
from pathlib import Path
import struct
import zlib

def convert(path):
    magic, dimensions, maximum, pixels = path.read_bytes().split(b"\n", 3)
    if magic != b"P6" or maximum != b"255":
        raise ValueError(f"{path}: expected P6 RGB8")
    width, height = map(int, dimensions.split())
    stride = width * 3
    if len(pixels) != stride * height:
        raise ValueError(f"{path}: incorrect pixel count")
    def chunk(kind, data):
        body = kind + data
        return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body))
    rows = b"".join(b"\0" + pixels[y*stride:(y+1)*stride] for y in range(height))
    png = (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
           + chunk(b"IDAT", zlib.compress(rows)) + chunk(b"IEND", b""))
    with path.with_suffix(".png").open("xb") as output:
        output.write(png)
    return path.with_suffix(".png")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    paths = sorted(args.directory.glob("*.ppm"))
    for path in paths:
        if not path.with_suffix(".png").exists():
            convert(path)
    print(f"PNG captures available: {len(paths)}")
