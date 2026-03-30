# Layout Strategy

## Partitioning

- Physically partition RF front-end, demodulator, bridge digital, and power sections.
- Keep RF path short and impedance controlled.
- Maintain clean analog ground reference near tuner input path.

## Grounding

- Use a continuous ground reference plane with carefully planned return paths.
- Avoid uncontrolled splits under high-speed or RF-critical traces.
- Use stitching vias around RF and clock perimeter zones.

## High-Speed and Control Routing

- Route USB differential pair with impedance and length matching constraints.
- Keep control buses away from RF input edge where practical.
- Prioritize access to debug lines for first revision.
