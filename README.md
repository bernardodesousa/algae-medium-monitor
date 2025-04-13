# Algae Medium Monitor

A Rust embedded project for monitoring and calibrating pH and temperature of algae cultures.

## Project Structure

The project has been organized into modules for better maintainability:

- **main.rs**: Main program flow and loop
- **adc.rs**: ADC functionality for reading analog sensors
- **uart.rs**: UART communication for sending data to a host computer 
- **ds18b20.rs**: DS18B20 temperature sensor interface (Dallas 1-Wire protocol)
- **ph.rs**: pH sensor calibration and conversion functions

## Hardware

- **Arduino Pro Mini** (ATmega328P) as the main controller
- **pH Sensor Module** with electrode, connected to ADC0
- **DS18B20 Temperature Sensor** connected to Port D2 for water temperature monitoring
- **UART** output at 9600 baud for debugging and data logging

## Features

- **Real-time pH monitoring** with calibration functionality
- **Temperature sensing** using a DS18B20
- **Calibration mode** for pH using references (lime juice, water, baking soda)
- **Serial output** for viewing data on a computer

## Calibration

The pH sensor can be calibrated using the following method:

1. Set `CALIBRATION_MODE = true` in ph.rs
2. Prepare calibration solutions (lime juice ~pH 2.2, water ~pH 7.0, baking soda ~pH 8.4)
3. Adjust the reference potentiometer on the pH module to match known pH values
4. Note that higher ADC values correspond to lower pH values (inverted relationship)

## Building and Flashing

```bash
# Build the project
.\build.ps1

# Flash to Arduino
.\flash.ps1
```

## Serial Output

Connect to the Arduino's serial port at 9600 baud to see the measurements:

- pH values (with raw ADC readings)
- Temperature values (from the DS18B20 digital sensor) 
- Temperature values from the pH module's T1 output (if connected)

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

