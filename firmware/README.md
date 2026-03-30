# Firmware Scaffold

This firmware project provides a buildable C/CMake skeleton for reference-platform bring-up and control-plane integration.

## Implemented Scaffold Elements

- BSP initialization flow
- Control interface abstraction
- Tuner abstraction
- Demodulator abstraction
- Bridge abstraction
- Structured logging hook
- Self-test hook
- Host-build unit-test target

## Build

```bash
cd firmware
cmake -S . -B build
cmake --build build
ctest --test-dir build --output-on-failure
```

## Directory Layout

- `include/fw/`: public firmware interfaces
- `src/`: core bring-up/control implementation
- `boards/`: board-level configuration placeholders
- `drivers/`: low-level driver stubs
- `tests/`: host-build unit tests
