# Hardware Requirements

## Functional Requirements

- Accept 75 ohm coax RF input suitable for ISDB-T terrestrial reception contexts.
- Protect RF input from ESD/transient events.
- Provide tuner/demodulator module integration points with documented control buses.
- Expose host data/control over a practical prototype interface.
- Support debug instrumentation (UART/SWD/I2C test access).

## Non-Functional Requirements

- Modular architecture supporting evaluation modules first.
- Bring-up friendly with explicit test points and rail observation.
- Power architecture supporting analog/digital isolation strategy.
- Layout strategy that can evolve toward EMI/EMC compliance.

## Host Bridge Trade-Offs

### USB

- Pros: native support on macOS, common tooling, high throughput, bus-powered options.
- Cons: firmware complexity for robust USB streaming.

### PCIe

- Pros: high throughput and low latency.
- Cons: higher hardware complexity, weak fit for quick external prototype path on macOS.

### Ethernet

- Pros: electrical isolation options, long cable support.
- Cons: extra protocol and latency overhead for local demo.

### Preferred for This Project

**USB 3.0 is preferred** for MVP because it balances throughput, host compatibility, and prototype integration speed.

## Placeholder Component Selection Criteria

### Tuner Module Criteria

- Frequency coverage compatible with target broadcast bands.
- Documented control interface and initialization sequence.
- Availability of evaluation board or reference design.
- Supply-chain viability (multiple distributors, lifecycle visibility).

### Demodulator Module Criteria

- Explicit ISDB-T mode support in vendor documentation.
- Accessible diagnostics metrics (lock state, BER-like indicators).
- Stable transport stream output path compatible with bridge.
- Known interoperability with candidate tuner modules.
