"""Generate the original parchment/brass pointer; no external image assets."""
from pathlib import Path
import struct
import zlib

def inside(x, y, polygon):
    hit = False
    for (a,b),(c,d) in zip(polygon, polygon[1:]+polygon[:1]):
        if (b>y)!=(d>y) and x<(c-a)*(y-b)/(d-b)+a:
            hit=not hit
    return hit

def generate():
    outer=[(2,2),(2,29),(9,23),(15,34),(21,31),(15,20),(26,19)]
    inner=[(4,6),(4,25),(10,20),(16,31),(18,30),(12,18),(21,17)]
    pixels=bytearray()
    for y in range(40):
        pixels.append(0)
        for x in range(32):
            color=(0,0,0,0)
            if inside(x-1,y-2,outer):color=(20,15,9,120)
            if inside(x,y,outer):color=(72,49,24,255)
            if inside(x,y,inner):color=(248,230,179,255)
            pixels.extend(color)
    def chunk(k,v):
        return struct.pack('>I',len(v))+k+v+struct.pack('>I',zlib.crc32(k+v))
    data=b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',32,40,8,6,0,0,0))+chunk(b'IDAT',zlib.compress(pixels))+chunk(b'IEND',b'')
    path=Path(__file__).resolve().parents[1]/'assets'/'cursor.png'
    path.write_bytes(data)

if __name__=='__main__':generate()
