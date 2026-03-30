# Risk Register

| Risk | Likelihood | Impact | Mitigation |
| --- | --- | --- | --- |
| RF front-end performance shortfall (sensitivity/selectivity) | Medium | High | Start with evaluation modules; run early bench characterization; keep front-end matching network tunable. |
| Demodulator compatibility risk with selected tuner modules | Medium | High | Define explicit interface contract; validate module pair in split-board prototype before custom integration. |
| Host interface bandwidth/latency mismatch | Low | Medium | Use USB 3.0 as preferred MVP link; instrument packet and latency metrics in backend. |
| macOS integration instability | Medium | Medium | Keep host APIs transport-agnostic; test on macOS and Linux in CI and local smoke runs. |
| Driver support gaps for bridge/eval modules | Medium | High | Favor modules with documented host support; keep replay/simulation path independent from vendor drivers. |
| EMI/EMC failures in prototype | Medium | High | Plan ground partitioning, shielding, and return-path control; execute pre-scan before respin. |
| Supply chain volatility for RF components | High | Medium | Track alternates in BOM; prioritize components with multiple distributors and package options. |
| Compliance/licensing constraints block real RF demos | Medium | High | Keep legal lab mode runnable; document external dependencies and licensing gate upfront. |
| Validation coverage insufficient for integration confidence | Medium | Medium | Define staged test matrix and acceptance criteria in validation plan; automate host-side checks. |
| Schedule slippage due to hardware respin | Medium | High | Use split-board/eval strategy first; freeze interfaces before custom PCB revision. |
| Prototype re-spin due to clock/power integrity issues | Medium | High | Simulate and measure rails/clocks early; include debug headers and measurement test points. |
