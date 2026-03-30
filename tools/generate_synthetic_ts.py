#!/usr/bin/env python3
"""Generate a deterministic MPEG-TS-like binary stream for lab replay tests."""

from __future__ import annotations

import argparse
from pathlib import Path

TS_PACKET_SIZE = 188
TS_SYNC_BYTE = 0x47


def build_packet(pid: int, continuity_counter: int, seed: int) -> bytes:
    packet = bytearray(TS_PACKET_SIZE)
    packet[0] = TS_SYNC_BYTE
    packet[1] = 0x40 | ((pid >> 8) & 0x1F)
    packet[2] = pid & 0xFF
    packet[3] = 0x10 | (continuity_counter & 0x0F)

    for i in range(4, TS_PACKET_SIZE):
        packet[i] = (seed + i + continuity_counter) & 0xFF

    return bytes(packet)


def generate(output: Path, packets: int, pid: int) -> tuple[int, int]:
    output.parent.mkdir(parents=True, exist_ok=True)

    with output.open("wb") as f:
        for idx in range(packets):
            cc = idx % 16
            f.write(build_packet(pid=pid, continuity_counter=cc, seed=idx & 0xFF))

    size = output.stat().st_size
    return packets, size


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Generate deterministic synthetic TS stream")
    parser.add_argument("--output", type=Path, required=True, help="Output TS file path")
    parser.add_argument("--packets", type=int, default=5000, help="Number of 188-byte packets")
    parser.add_argument("--pid", type=lambda x: int(x, 0), default=0x30, help="Packet PID")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.packets <= 0:
        raise SystemExit("--packets must be > 0")

    packets, size = generate(args.output, args.packets, args.pid)
    print(f"generated {packets} packets ({size} bytes) at {args.output}")


if __name__ == "__main__":
    main()
