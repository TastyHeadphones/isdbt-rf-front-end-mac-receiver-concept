# Block Diagrams

This document provides explicit diagrams for hardware, signal flow, control flow, power architecture, runtime modes, and hardware/software interaction.

## 1. Top-Level Hardware Block Diagram

```mermaid
flowchart LR
  A["75 ohm Coax Input"] --> B["ESD and Surge Protection"]
  B --> C["RF Band Select and Matching Network"]
  C --> D["Tuner Front-End Module"]
  D --> E["ISDB-T Demodulator Module"]
  E --> F["Bridge Controller MCU/FPGA"]
  F --> G["USB 3.0 Device Interface"]
  G --> H["macOS Host"]
  I["Clock Source 16-30.4 MHz"] --> D
  I --> E
  J["Power Tree"] --> D
  J --> E
  J --> F
  K["Debug Header (SWD/UART/I2C)"] --> F
```

## 2. Signal Path Diagram

```mermaid
flowchart LR
  subgraph RF["RF Capture Path"]
    A["Coax RF In"] --> B["Protection and Matching"]
    B --> C["Tuner"]
    C --> D["Demodulator"]
  end

  subgraph Host["Host Transport Path"]
    D --> E["Transport Stream Framing"]
    E --> F["Bridge over USB"]
    F --> G["host-backend Intake"]
    G --> H["Capture/Replay Store"]
    G --> I["Metrics and Diagnostics"]
    G --> J["Optional Player/Inspector"]
  end

  subgraph Test["Test Injection"]
    K["Synthetic TS Generator"] --> G
    L["Legal Prerecorded TS"] --> G
    M["SDR/Dev Board Test RF"] --> B
  end
```

## 3. Control Path Diagram

```mermaid
flowchart TB
  UI["Tauri UI"] --> API["host-backend Control API"]
  CLI["isdbtctl CLI"] --> API
  API --> CFG["Configuration Manager (TOML)"]
  API --> DEV["Device Discovery Manager"]
  API --> TUNE["Tune Controller"]
  DEV --> BRIDGE["USB Bridge Session"]
  TUNE --> BRIDGE
  BRIDGE --> FW["Firmware Control Layer"]
  FW --> TUNER["Tuner Control (I2C/SPI)"]
  FW --> DEMOD["Demodulator Control (I2C/SPI)"]
  API --> LOGS["Structured Logs and Telemetry"]
```

## 4. Power Tree Diagram

```mermaid
flowchart TD
  VIN["USB-C 5V Input / Bench 5V"] --> PROT["Input Protection + Fuse"]
  PROT --> SW1["Buck 3.3V Digital"]
  PROT --> SW2["LDO 3.3V RF Quiet"]
  PROT --> SW3["Buck/LDO 1.8V Core"]
  PROT --> SW4["Optional 1.2V Core"]
  SW1 --> MCU["Bridge MCU/FPGA"]
  SW1 --> IO["I/O Pullups and Control"]
  SW2 --> TUNER["Tuner Analog Rails"]
  SW2 --> DEMODA["Demodulator Analog Rails"]
  SW3 --> DEMODD["Demodulator Digital Rails"]
  SW4 --> CORE["High-Speed Core (if required)"]
```

## 5. Lab Demo Mode Diagram

```mermaid
flowchart LR
  A["Synthetic TS Generator"] --> D["host-backend"]
  B["Legal Prerecorded TS"] --> D
  C["Optional USB Eval Module"] --> D
  D --> E["Metrics and Diagnostics"]
  D --> F["Capture File"]
  G["isdbtctl"] --> D
  H["Tauri UI"] --> D
  I["Automated Tests"] --> D
```

## 6. Real RF Integration Mode Diagram

```mermaid
flowchart LR
  A["Coax RF Input"] --> B["RF Protection and Matching"]
  B --> C["Tuner Eval Module or Custom Front-End"]
  C --> D["ISDB-T Demodulator Eval Module"]
  D --> E["Bridge Controller Firmware"]
  E --> F["USB/Ethernet Host Link"]
  F --> G["host-backend"]
  G --> H["UI/CLI"]
  X["Licensed Conditional Access Components (External)"] -. "Compliant ecosystem integration only" .-> G
```

## 7. Hardware/Software Interaction Diagram

```mermaid
sequenceDiagram
  participant UI as Tauri UI
  participant CLI as isdbtctl
  participant HB as host-backend
  participant BR as Bridge Controller
  participant FW as Firmware BSP
  participant TU as Tuner
  participant DM as Demodulator

  UI->>HB: Request device status
  CLI->>HB: Tune command frequency and bandwidth
  HB->>BR: Open control channel
  BR->>FW: Dispatch tune command
  FW->>TU: Program RF frequency
  FW->>DM: Configure demodulation profile
  DM-->>FW: Lock metrics and BER
  FW-->>BR: Status update
  BR-->>HB: Telemetry frame
  HB-->>UI: Signal metrics and lock state
```

Source files are mirrored in [diagrams/](/Users/young/Github/isdbt-rf-front-end-mac-receiver-concept/diagrams).
