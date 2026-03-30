# Power Tree

## Input Source

- Primary prototype input: USB-C 5V or bench 5V rail.
- Input protection: resettable fuse + reverse polarity/transient handling as applicable.

## Rails

- `3V3_DIG`: bridge MCU/FPGA and digital control domain.
- `3V3_RF`: quiet analog rail for tuner and sensitive demod analog blocks.
- `1V8_CORE`: demod/bridge core logic where required.
- `1V2_CORE` (optional): high-speed core rail for selected bridge silicon.

## Regulator Strategy

- Buck for higher-current digital rail.
- Low-noise LDO post-regulation for RF analog-sensitive domains.
- Dedicated enable sequencing under firmware or supervisor control when required.

## Reset Strategy

- Hardware reset supervisor for deterministic power-on reset.
- Individual reset lines for tuner/demod/bridge blocks.
- Test-accessible reset controls for bring-up.
