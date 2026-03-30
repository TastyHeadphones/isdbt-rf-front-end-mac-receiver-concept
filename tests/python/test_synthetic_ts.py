from pathlib import Path

from tools.generate_synthetic_ts import TS_PACKET_SIZE, TS_SYNC_BYTE, generate


def test_synthetic_ts_generation(tmp_path: Path) -> None:
    output = tmp_path / "sample.ts"
    packets, size = generate(output=output, packets=256, pid=0x30)

    assert packets == 256
    assert size == 256 * TS_PACKET_SIZE

    payload = output.read_bytes()
    assert payload[0] == TS_SYNC_BYTE

    for offset in range(0, len(payload), TS_PACKET_SIZE):
        assert payload[offset] == TS_SYNC_BYTE
