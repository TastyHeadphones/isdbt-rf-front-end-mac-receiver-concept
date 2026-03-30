# System Architecture

## Purpose

This document defines a concrete, implementation-oriented architecture for `isdbt-mac-receiver-reference-platform`, with a runnable Lab Demo Mode and a bounded Real RF Integration Mode.

## End-to-End Architecture

1. RF input path accepts 75 ohm coax input in terrestrial broadcast frequency ranges.
2. Input protection stage applies ESD/surge protection and controlled impedance entry.
3. Tuner/front-end stage selects channel and down-converts/filters for demodulation.
4. Demodulator stage recovers transport stream payload and signal quality metrics.
5. Bridge stage packages stream and control/telemetry over host interface (preferred: USB 3.0 for MVP).
6. Host backend receives stream/control data, exposes APIs, logs telemetry, and supports replay/simulation.
7. CLI and desktop UI drive control, diagnostics, capture/replay, and demo operations.

## Data Plane

- **Primary path (integration):** Coax RF -> protection/matching -> tuner -> demodulator -> bridge -> host backend.
- **Lab path (mandatory runnable):** synthetic/legal prerecorded TS -> host backend intake abstraction -> metrics/capture/view.
- **Capture/replay path:** host backend can ingest file input, expose packet counters, and write capture artifacts.
- **Optional inspection path:** transport stream can be forwarded to optional analysis tools via plugin hooks.

## Control Plane

- UI and CLI call host backend control endpoints.
- Host backend maintains device registry and runtime config.
- Backend sends tune/config commands through bridge abstraction.
- Firmware maps control commands to tuner/demodulator register transactions.
- Telemetry (lock state, SNR proxy, BER proxy, packet counters) is published as structured events.

## Hardware and Software Split

### Hardware Responsibilities

- RF connector, protection, and matching.
- Tuner and demodulator module hosting.
- Bridge controller and physical host link.
- Clock, reset, power regulation, and debug access.

### Firmware Responsibilities

- BSP initialization and power-up sequencing hooks.
- Tuner/demodulator control abstraction.
- Bridge command handling.
- Self-test and telemetry hooks.

### Host Software Responsibilities

- Device discovery and session lifecycle.
- Configuration and profile management.
- Stream intake abstraction (hardware stream or replay file).
- Diagnostics metrics and log collection.
- Demo orchestration APIs for CLI/UI.

## Runtime Modes

### Lab Demo Mode (Runnable)

- Uses simulated device manager and file/synthetic stream input.
- Exercises end-to-end control and telemetry flow without licensed protected-access modules.
- Supports optional USB evaluation path when available.

### Real RF Integration Mode (Documented, Bounded)

- Defines electrical/mechanical/logical interfaces for integrating tuner/demodulator hardware.
- Requires legally compliant external components for protected reception contexts.
- Excludes conditional-access bypass implementations.

## Explicit Assumptions

- A legal test stream source exists for demonstration (synthetic or legally distributed test TS).
- Evaluation modules provide documented register/control interfaces where used.
- Host runs on macOS or Linux for development; CI runs on Linux.
- Final production compliance and certification are outside this scaffold stage.

## Out-of-Scope Boundaries

- B-CAS/ACAS circumvention, key extraction, or unauthorized decryption.
- Production-grade RF certification, enclosure thermal design, and mass-manufacturing artifacts.
- Complete product UI/UX polish for consumer release.

## Explicit Interfaces

- **RF:** 75 ohm coax entry, ESD/surge front-end, impedance-controlled route to tuner.
- **Control buses:** I2C/SPI for tuner/demodulator configuration.
- **Host bridge:** USB 3.0 preferred for MVP throughput and plug-and-play on macOS.
- **Host API:** HTTP/JSON endpoints for discovery, tune, metrics, replay control.
- **Config:** TOML runtime configuration.
- **Logging:** JSON structured logs via `tracing`.

## How this project is guaranteed to run

### Exact Runnable Subset

- `software/host-backend`: starts and serves discovery/tune/metrics/replay endpoints.
- `software/cli`: exercises discovery, tune, replay, and metrics flows.
- `software/ui`: diagnostics/signal metrics skeleton consuming backend API.
- `tools/generate_synthetic_ts.py`: generates deterministic MPEG-TS-like packets for replay.
- `tests/python`: validates synthetic stream and config-level behaviors.

### What Is Mocked

- Device enumeration defaults to simulated device identity in Lab Demo Mode.
- RF lock/SNR/BER are modeled values unless evaluation hardware is attached.

### What Is Replayed

- Synthetic TS generated locally.
- Legal prerecorded test TS files supplied by user/team policy.

### What Can Execute Without Licensed Protected-Access Modules

- Backend discovery/tune command path.
- Replay ingestion and metrics.
- CLI/UI diagnostics.
- Logging, capture, and validation scripts.

No protected-content access implementation is required for this runnable subset.
