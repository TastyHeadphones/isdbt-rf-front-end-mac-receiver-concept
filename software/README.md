# Software Stack

This folder contains the runnable host software path for Lab Demo Mode and the integration scaffolding for hardware-connected operation.

## Components

- `host-backend` (Rust): device discovery, tune control, replay intake, metrics, structured logging
- `cli` (Rust): command-line operations against backend API
- `shared` (Rust): shared config and protocol types
- `ui` (Tauri + TypeScript skeleton): diagnostics and signal metrics UI
- `docs/`: API and interface notes

## Lab Demo Run

```bash
cd software
cargo run -p host-backend -- --config shared/config/default.toml
```

In another terminal:

```bash
cd software
cargo run -p isdbtctl -- discover
cargo run -p isdbtctl -- tune --device sim-demod-001 --frequency-hz 569142857 --bandwidth-mhz 6
cargo run -p isdbtctl -- replay --file ../tests/data/synthetic_demo.ts
cargo run -p isdbtctl -- metrics
```

## API Reference

See [software/docs/api_interfaces.md](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/software/docs/api_interfaces.md).
