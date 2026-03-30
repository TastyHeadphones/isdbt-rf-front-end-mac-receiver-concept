# Validation Plan

## Objectives

- Prove Lab Demo Mode is runnable and repeatable.
- Validate control-plane behavior (discover, tune, metrics, replay).
- Prepare integration gates for hardware bring-up.

## Unit-Test Scope

- Rust: config parsing, API models, replay accounting helpers.
- Firmware: init and control abstraction stubs through host-build tests.
- Python: synthetic stream generation and file-structure checks.
- TypeScript: UI type checks and API client compile checks.

## Integration-Test Scope

- Start backend in Lab Demo Mode.
- Discover simulated device via CLI.
- Issue tune command and verify lock-state telemetry transition.
- Replay legal/synthetic TS file and validate packet counters increase.
- Verify metrics endpoint and UI polling path operate.

## Hardware-in-the-Loop Future Scope

- Replace simulated device with evaluation module path.
- Validate tuner/demod register transactions over bridge.
- Validate TS integrity over host interface under sustained load.

## Bring-Up Milestones

1. Host-only replay path operational.
2. Simulated control-plane loop complete.
3. Evaluation module enumeration operational.
4. RF injection and lock metrics measurable.
5. Integrated demo script passes acceptance criteria.

## Lab Demo Acceptance Criteria

- Backend starts with no hardware attached.
- CLI discover shows at least one simulated device.
- Tune command completes and returns lock-state response.
- Replay input is ingested and packet counters are non-zero.
- Logs are emitted in structured format and include control+metrics events.
- UI diagnostics page displays device and metrics data.

## Successful End-to-End Run Definition

A successful end-to-end run in this repository means:

- A legal/synthetic test stream is processed by host backend.
- Control-plane operations (discover/tune/replay/metrics) execute without manual code changes.
- Output evidence (logs + CLI output + optional capture artifact) confirms flow completion.

Protected commercial stream access is not required for success.
