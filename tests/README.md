# Test Strategy

## Scope

- Rust unit tests for shared config/protocol components.
- Firmware host-build unit tests for init/tune/self-test scaffold.
- Python tests for synthetic TS generation and repository runnable-contract checks.
- Integration smoke path via `tools/lab_demo_smoke.py` when backend is running.

## Current Test Commands

```bash
# Rust
cd software && cargo test --workspace

# Firmware
cd firmware && cmake -S . -B build && cmake --build build && ctest --test-dir build --output-on-failure

# Python
python3 -m pip install -r tools/requirements.txt
python3 -m pytest tests/python
```

## Future Expansion

- Hardware-in-the-loop tests with evaluation module connected.
- Transport integrity and sustained-rate stress tests.
- Automated UI smoke checks against backend.
