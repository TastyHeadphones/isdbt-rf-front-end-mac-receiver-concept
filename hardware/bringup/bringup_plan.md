# Bring-Up Plan

## Bring-Up Sequence

1. Visual inspection and continuity checks.
2. Power-on with current-limited bench supply.
3. Validate rails (`3V3_DIG`, `3V3_RF`, `1V8_CORE`, optional `1V2_CORE`).
4. Validate reset supervisor timing and manual reset behavior.
5. Validate clocks at designated test points.
6. Program bridge firmware and verify debug console output.
7. Verify control bus transactions to tuner/demod placeholders.
8. Validate host enumeration over USB.
9. Run host backend discovery and tune flow.
10. Introduce RF or test-injection signal path and capture metrics.

## Gate Criteria per Step

- No thermal anomalies on regulators.
- Stable rails within tolerance.
- Deterministic reset behavior.
- Confirmed bridge communication with host.
- Confirmed control responses from modules.
