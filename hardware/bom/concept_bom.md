# Concept BOM

## Purpose

This BOM is a concept baseline with placeholder categories and selection criteria. It is not a final purchasing list.

## Core Categories

| Category | Placeholder | Notes |
| --- | --- | --- |
| RF connector | `J_RF_IN` | 75 ohm coax connector, panel/lab compatible |
| ESD/surge | `D_ESD_RF`, `TVS_RF` | RF-suitable low-capacitance protection |
| Matching network | `L/C/R_MAT_*` | Populate options for tuning during bring-up |
| Tuner module | `TUNER_X1` | Select via documented ISDB-T support + eval board availability |
| Demod module | `DEMOD_X1` | Select via ISDB-T support + metrics visibility |
| Bridge controller | `BRIDGE_X1` | USB 3.0-capable MCU/FPGA class component |
| Regulators | `REG_3V3_DIG`, `REG_3V3_RF`, `REG_1V8` | Include low-noise path for RF analog rail |
| Oscillator | `XOSC_REF` | Low-jitter reference for demod-sensitive path |
| Debug headers | `J_SWD`, `J_UART`, `TP_*` | Bring-up access mandatory |

## Supply Chain Strategy

- Track primary + alternate part numbers for all high-risk items.
- Avoid single-source components where practical.
- Record lifecycle status before BOM freeze.
