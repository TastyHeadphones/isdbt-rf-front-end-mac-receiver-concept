# isdbtctl

CLI client for host-backend APIs.

Examples:

```bash
cd software
cargo run -p isdbtctl -- discover
cargo run -p isdbtctl -- tune --device sim-demod-001 --frequency-hz 569142857 --bandwidth-mhz 6
cargo run -p isdbtctl -- replay --file ../tests/data/synthetic_demo.ts
cargo run -p isdbtctl -- metrics
```
