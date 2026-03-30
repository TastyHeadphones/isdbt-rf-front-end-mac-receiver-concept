# Clocking Concept

## Clock Domains

- Reference oscillator domain for tuner and/or demodulator.
- Bridge processing clock domain.
- Optional USB PHY reference as required by bridge implementation.

## Clock Requirements

- Use low-jitter oscillator for demod-sensitive path.
- Route clocks with controlled return paths and short stubs.
- Isolate noisy digital switching from RF-sensitive reference traces.

## Bring-Up Clock Validation

- Expose measurement points for primary clock lines.
- Validate frequency and amplitude before demod initialization.
