# host-backend

Rust service providing:

- device discovery abstraction
- tune command handling
- transport stream replay intake
- metrics and health endpoints
- structured logging for diagnostics

Run:

```bash
cd software
cargo run -p host-backend -- --config shared/config/default.toml
```
