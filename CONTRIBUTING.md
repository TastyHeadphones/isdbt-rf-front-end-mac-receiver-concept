# Contributing

## Principles

- Keep design decisions explicit and technically testable.
- Maintain a runnable Lab Demo Mode.
- Respect all compliance boundaries: no conditional-access circumvention.

## Development Workflow

1. Create a feature branch.
2. Implement scoped changes with tests or test placeholders.
3. Run local checks:

```bash
# Rust
cd software && cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace

# Firmware
cd firmware && cmake -S . -B build && cmake --build build && ctest --test-dir build --output-on-failure

# Python
python3 -m pip install -r tools/requirements.txt
python3 -m pytest tests/python

# TypeScript
cd software/ui && npm install && npm run typecheck
```

4. Submit a pull request with design rationale and test evidence.

## Commit and PR Expectations

- Use clear commit messages.
- Include impacted subsystem tags when practical (`hardware`, `firmware`, `software`, `docs`, `tests`).
- Update documentation when interfaces change.

## Compliance Rules

Contributions that attempt to bypass B-CAS/ACAS, protected broadcast controls, or encryption protections will be rejected. This repository is for legal and compliant development and demonstration only.
