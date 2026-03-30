# Lab Demo Mode Runbook

## Purpose

This runbook defines the mandatory runnable path for the repository without requiring licensed protected-access modules.

## Preconditions

- Rust toolchain installed.
- Python 3.11+ installed.
- Optional Node.js 20+ for UI.

## Demo Flow

1. Generate synthetic transport stream:

```bash
python3 tools/generate_synthetic_ts.py --output tests/data/synthetic_demo.ts --packets 5000
```

2. Start backend in Lab Demo Mode:

```bash
cd software
cargo run -p host-backend -- --config shared/config/default.toml
```

3. Discover simulated device:

```bash
cd software
cargo run -p isdbtctl -- discover
```

4. Send tune command:

```bash
cd software
cargo run -p isdbtctl -- tune --device sim-demod-001 --frequency-hz 569142857 --bandwidth-mhz 6
```

5. Replay stream and check metrics:

```bash
cd software
cargo run -p isdbtctl -- replay --file ../tests/data/synthetic_demo.ts
cargo run -p isdbtctl -- metrics
```

6. Optional UI diagnostics:

```bash
cd software/ui
npm install
npm run dev
```

## Evidence to Capture

- Backend startup logs.
- CLI outputs for discover/tune/metrics.
- Replay completion log and packet counter delta.
- Optional UI screenshot of diagnostics and metrics.

## Acceptance Check

Lab demo is accepted when:

- Simulated device is enumerated.
- Tune request returns lock-state response.
- Replay increments packet counters.
- Metrics endpoint remains responsive during replay.

## Legal Boundary Reminder

This flow uses synthetic/legal test streams and simulated/evaluation paths only. It does not include protected-access circumvention.
