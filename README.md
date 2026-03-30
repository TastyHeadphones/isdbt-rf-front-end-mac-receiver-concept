# isdbt-mac-receiver-reference-platform

## Project Guarantee

This repository guarantees a runnable lab/demo workflow, concrete hardware and software architecture, explicit implementation scaffolding, and a documented path toward prototype integration. It does not guarantee protected broadcast access without licensed and compliant ecosystem components.

## Summary

`isdbt-mac-receiver-reference-platform` is a concept-to-prototype engineering reference for a Japan ISDB-T receiver workflow targeting lab demonstration and integration planning on macOS host systems. It is implementation-oriented: hardware structure, firmware scaffolding, host software stack, test automation, and bring-up/validation flows are all included.

## Project Goals

- Provide a technically credible hardware and software reference architecture for an ISDB-T receiver platform.
- Deliver a **runnable** Lab Demo Mode with no protected-access circumvention.
- Define explicit interfaces between RF front-end, demodulation layer, bridge path, firmware, backend, CLI, and UI.
- Document a bounded Real RF Integration Mode for compliant licensed ecosystem integration.
- Enable engineers to execute bring-up, validation, and demo workflows from this repository baseline.

## What “Runnable” Means Here

A runnable repository means:

- The software stack (`host-backend`, `cli`, `ui` skeleton) can run in Lab Demo Mode.
- Synthetic or legally obtained test transport streams can be replayed through the host pipeline.
- Device discovery, tune command flow, logging, and diagnostics can be exercised with simulated or evaluation-path inputs.
- Test scripts and CI can execute without requiring protected broadcast access components.

## Supported Modes

### 1) Lab Demo Mode (Mandatory, Runnable)

- Host-side replay/simulation mode is fully runnable.
- Supports synthetic TS generation and legal prerecorded TS replay.
- Supports simulated enumeration/tune flow and metrics reporting.
- Can be extended to USB-connected evaluation modules without changing the control-plane architecture.

### 2) Real RF Integration Mode (Documented, Bounded)

- Documents how coax RF input, protection, tuner, demodulator, and host bridge connect.
- Keeps legal/compliance boundaries explicit.
- Does **not** implement protected-access bypass or unlicensed decryption workflows.

## Scope

- Reference hardware partitioning, power, clocks, reset, interfaces, and bring-up planning.
- Firmware scaffolding for board bring-up and control abstractions.
- Rust backend + Rust CLI + Tauri/TypeScript UI skeleton.
- Validation plans, risk register, roadmap, and compliance boundaries.

## Non-Goals

- Shipping a production-certified receiver product.
- Implementing or reverse-engineering conditional access systems.
- Delivering guaranteed compatibility with unspecified commercial protected streams.

## Legal and Compliance Boundary

- Protected broadcast reception may require licensed and compliant ecosystem components.
- No B-CAS/ACAS circumvention workflow is implemented.
- Lab demo operation uses legal test streams, synthetic streams, development modules, or unprotected/internal flows where applicable.
- See [docs/compliance_and_legal.md](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/docs/compliance_and_legal.md).

## Tech Stack

- Documentation: Markdown + Mermaid
- Hardware docs: KiCad 8 style project structure
- Firmware scaffold: C + CMake
- Host backend: Rust
- Desktop UI: Tauri + TypeScript skeleton
- CLI: Rust
- Test automation: Python + pytest
- Dev environment: Docker + devcontainer
- CI: GitHub Actions
- Config: TOML
- Logging: structured logging (`tracing`)

## Repository Structure

- [docs/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/docs)
- [diagrams/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/diagrams)
- [hardware/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/hardware)
- [firmware/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/firmware)
- [software/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/software)
- [tools/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/tools)
- [tests/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/tests)
- [.github/workflows/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/.github/workflows)

## Quick Start

1. Build and run host backend (Lab Demo Mode):

```bash
cd software
cargo run -p host-backend -- --config shared/config/default.toml
```

2. In another shell, discover and tune simulated device:

```bash
cd software
cargo run -p isdbtctl -- discover
cargo run -p isdbtctl -- tune --device sim-demod-001 --frequency-hz 569142857 --bandwidth-mhz 6
```

3. Generate a synthetic transport stream and replay:

```bash
python3 tools/generate_synthetic_ts.py --output tests/data/synthetic_demo.ts --packets 5000
cd software
cargo run -p isdbtctl -- replay --file ../tests/data/synthetic_demo.ts
cargo run -p isdbtctl -- metrics
```

4. Open UI skeleton:

```bash
cd software/ui
npm install
npm run dev
```

See [docs/lab_demo.md](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/docs/lab_demo.md) for the full demo flow.

## Development Status

- Phase: early reference-platform scaffold
- Hardware: concept-level architecture and KiCad project structure defined
- Firmware: buildable bring-up/control skeleton
- Software: runnable backend + CLI lab path, UI skeleton ready for integration

## Roadmap

See [docs/roadmap.md](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/docs/roadmap.md).

## Contribution Notes

See [CONTRIBUTING.md](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/CONTRIBUTING.md). Contributions must preserve legal compliance boundaries and must not introduce access-control circumvention.

## What signal is on a Japanese apartment wall outlet?

The wall outlet is typically **RF over coaxial cable**, carrying broadcast RF channels in frequency bands used for terrestrial or cable distribution. In this project context, terrestrial reception means handling an ISDB-T RF broadcast signal path at tuner/demodulator level. It is **not HDMI**, and it is **not already-decoded video**. Any protected reception path may require licensed ecosystem components (for example, approved conditional-access handling) that are outside the scope of this repository.
