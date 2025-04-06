# Algae Medium Monitor

A Rust application for AVR microcontrollers that monitors pH and temperature of an algae water suspension.

The program uses sensors to monitor key parameters of algae cultivation and provides data for optimal growth conditions.

Designed for the ATmega328p on the Arduino Pro Mini and compatible boards.

[The AVR-Rust Book](https://book.avr-rust.org/)

## Key Features

- pH monitoring of algae suspension
- Temperature monitoring of cultivation medium
- Data logging capabilities
- Designed for long-term reliability and accuracy

## Prerequisites

* A recent version of the nightly Rust compiler.
* A recent version of Cargo.
* The rust-src rustup component - `$ rustup component add rust-src`
* AVR-GCC on the system for linking
* AVR-Libc on the system for support libraries
* WinAVR for Windows users (provides avr-gcc, avrdude, etc.)
* FTDI or compatible USB-to-Serial adapter for flashing

## Building Manually

To build manually, run:

```bash
# Ensure time delays are consistent with a 16MHz microcontroller.
export AVR_CPU_FREQUENCY_HZ=16000000

# Compile the crate to an ELF executable.
cargo build -Z build-std=core --release
```

## Using Build Scripts

This project includes PowerShell scripts to simplify the build and flash process:

1. `build.ps1` - Handles the build process, with a fallback to manual linking
2. `flash.ps1` - Flashes the compiled program to an Arduino Pro Mini

To build and flash:

```powershell
# Set the COM port for your Arduino (or let the script prompt you)
$env:AVR_COM_PORT = "COM7"  # Change to your COM port

# Build and flash in one step
.\flash.ps1
```

## Hardware Requirements

- Arduino Pro Mini or compatible AVR board
- pH sensor module (analog)
- DS18B20 or similar temperature sensor
- Optional: SD card module for data logging

## Troubleshooting

If you have trouble flashing the Arduino, see the `FLASHING_TIPS.md` file for detailed troubleshooting instructions.

## Resources

* The [AVR-Rust book](https://book.avr-rust.org)
* [Arduino Pro Mini documentation](https://docs.arduino.cc/hardware/pro-mini)

