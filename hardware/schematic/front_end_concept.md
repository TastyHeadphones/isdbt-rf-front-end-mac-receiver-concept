# Front-End Concept

## RF Input Connector Expectations

- Connector type: 75 ohm coaxial connector suitable for lab and fixture use.
- Entry network includes:
  - ESD diode network rated for RF front-end protection
  - surge/transient limiter strategy
  - DC blocking where required by selected modules

## Input Matching

- Maintain controlled 75 ohm path from connector to tuner input network.
- Provide footprint options for Pi/L matching adjustment during bring-up.
- Keep tuning pads accessible for VNA characterization.

## Tuner Placeholder Module Criteria

- ISDB-T relevant frequency support in documented range.
- External control via I2C/SPI with public register map references or NDA-ready docs.
- Evaluation board availability for pre-integration validation.

## Demodulator Placeholder Module Criteria

- ISDB-T demod capability with documented lock/quality outputs.
- TS output interface that can be mapped to bridge (parallel/serial/USB-side ingest path).
- Known reset and clock requirements.

## Test Signal Injection Path

- Optional injection header/jump path for SDR/dev-board generated test RF.
- Optional bypass for host-only replay mode without RF chain dependency.
