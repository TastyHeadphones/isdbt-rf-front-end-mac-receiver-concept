# Roadmap

## Phase 0: Requirements and Runnable-Scope Definition

- Freeze legal/compliance boundary.
- Define mandatory Lab Demo Mode acceptance criteria.
- Document bounded Real RF Integration Mode interfaces.

## Phase 1: System Architecture and Repository Scaffolding

- Complete architecture and block diagram package.
- Stand up firmware/software/hardware folder structures.
- Add CI, linting, formatting, and development environment scaffolding.

## Phase 2: Host Software Runnable Lab Path

- Implement backend discovery/tune/metrics/replay baseline.
- Implement CLI commands for end-to-end control flow.
- Implement UI diagnostics and signal metrics skeleton.
- Validate replay and simulation path in automated tests.

## Phase 3: Firmware Skeleton and Interface Stabilization

- Establish BSP init path and control abstractions.
- Define tuner/demodulator/bridge interfaces.
- Add self-test and logging hooks.
- Stabilize host-firmware control schema.

## Phase 4: Hardware Concept and BOM Freeze

- Finalize front-end partitioning and power strategy.
- Select evaluation modules and alternates.
- Freeze concept BOM and interface headers.

## Phase 5: Prototype Bring-Up Planning

- Execute bring-up sequence planning and checklist.
- Prepare lab equipment and fixture requirements.
- Define gate criteria for first hardware power-on.

## Phase 6: Integration Validation

- Integrate eval path with host stack.
- Run signal-path, control-path, and telemetry validation.
- Record known gaps and mitigation actions.

## Phase 7: Compliance Boundary Review

- Re-check legal/compliance statements against implementation.
- Confirm no circumvention capability was introduced.
- Document licensing and approval requirements for next stage.
