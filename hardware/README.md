# Hardware Reference

This folder defines a concrete hardware reference architecture for an ISDB-T receiver prototype workflow, with emphasis on bring-up practicality and legal demo operability.

## Hardware Scope

- Coax RF input handling (connector, protection, matching)
- Tuner and demodulator module integration strategy
- Host bridge path (preferred USB 3.0)
- Power, clocks, reset, debug visibility
- PCB partitioning, stack-up, EMI/EMC planning
- Bring-up sequencing and test point strategy

## Fastest Path to a Working Prototype

Recommended path for first executable prototype:

1. Use **evaluation modules** for tuner and demodulator rather than first-pass custom RF silicon integration.
2. Use a **split-board approach**:
   - RF/eval mezzanine area for tuner/demod experimentation.
   - Digital bridge board section for stable USB host connectivity.
3. Use a **known USB bridge approach** (MCU/FPGA + USB 3.0 device stack) to reduce host integration risk.
4. Ship MVP with:
   - host replay/simulation mode (already runnable from software)
   - optional eval-module live path
5. Freeze interfaces and power tree before custom PCB revision.

This approach minimizes re-spin risk while preserving a clear path to integrated hardware.
