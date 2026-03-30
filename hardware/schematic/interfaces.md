# Interfaces

## Control Interfaces

- `I2C0`: tuner control and status.
- `I2C1/SPI0`: demodulator control depending on selected module.
- `GPIO`: reset, interrupt, lock indication lines.

## Host Bridge Interface

- Preferred: USB 3.0 SuperSpeed device interface.
- Alternate documented path: Ethernet bridge for lab networks (non-MVP).

## Debug Ports

- SWD/JTAG header for bridge firmware debugging.
- UART console header for early boot diagnostics.
- Optional I2C/SPI test header for low-level register bring-up.
