#!/usr/bin/env python3
"""Generate the placeholder app icon (dark rounded square, accent stripes).

Usage: python3 scripts/gen_icon.py
Writes gui/src-tauri/icons/icon.png. Stdlib only — no PIL needed.
"""

import os
import struct
import zlib

SIZE = 512
ACCENT = (120, 81, 169, 255)  # royal purple — keep in sync with gui/src/styles.css
BG = (28, 31, 40, 255)        # dark panel


def px(x: int, y: int) -> tuple[int, int, int, int]:
    m = 48   # outer margin
    r = 80   # corner radius
    # rounded-rect mask
    cx = min(max(x, m + r), SIZE - m - r)
    cy = min(max(y, m + r), SIZE - m - r)
    in_corner = (x < m + r or x > SIZE - m - r) and (y < m + r or y > SIZE - m - r)
    if in_corner and (x - cx) ** 2 + (y - cy) ** 2 > r * r:
        return (0, 0, 0, 0)
    if x < m or x > SIZE - m or y < m or y > SIZE - m:
        return (0, 0, 0, 0)
    # three "text lines" in the accent color
    for ty, w in ((176, 320), (256, 256), (336, 288)):
        if ty <= y < ty + 40 and 96 <= x < 96 + w:
            return ACCENT
    return BG


def chunk(tag: bytes, data: bytes) -> bytes:
    return (
        struct.pack(">I", len(data))
        + tag
        + data
        + struct.pack(">I", zlib.crc32(tag + data))
    )


def main() -> None:
    rows = b""
    for y in range(SIZE):
        rows += b"\x00" + b"".join(bytes(px(x, y)) for x in range(SIZE))

    png = (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", SIZE, SIZE, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(rows, 9))
        + chunk(b"IEND", b"")
    )

    out = os.path.join(
        os.path.dirname(__file__), "..", "gui", "src-tauri", "icons", "icon.png"
    )
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "wb") as f:
        f.write(png)
    print(f"wrote {len(png)} bytes to {os.path.normpath(out)}")


if __name__ == "__main__":
    main()
