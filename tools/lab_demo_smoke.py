#!/usr/bin/env python3
"""Basic backend smoke test for discover/tune/replay/metrics endpoints."""

from __future__ import annotations

import argparse
from pathlib import Path

import requests


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run lab demo API smoke checks")
    parser.add_argument("--endpoint", default="http://127.0.0.1:8088")
    parser.add_argument("--ts-file", type=Path, required=True)
    return parser.parse_args()


def must(response: requests.Response) -> dict | list:
    if response.status_code >= 400:
        raise RuntimeError(f"HTTP {response.status_code}: {response.text}")
    return response.json()


def main() -> None:
    args = parse_args()
    base = args.endpoint.rstrip("/")

    devices = must(requests.get(f"{base}/api/v1/devices", timeout=4))
    if not devices:
        raise RuntimeError("No devices returned")

    device_id = devices[0]["id"]
    tune = must(
        requests.post(
            f"{base}/api/v1/tune",
            json={
                "device_id": device_id,
                "frequency_hz": 569142857,
                "bandwidth_mhz": 6,
            },
            timeout=4,
        )
    )
    if not tune.get("accepted"):
        raise RuntimeError(f"Tune rejected: {tune}")

    replay = must(
        requests.post(
            f"{base}/api/v1/replay",
            json={"file": str(args.ts_file)},
            timeout=20,
        )
    )

    metrics = must(requests.get(f"{base}/api/v1/metrics", timeout=4))

    print("smoke ok")
    print({"tune": tune, "replay": replay, "metrics": metrics})


if __name__ == "__main__":
    main()
