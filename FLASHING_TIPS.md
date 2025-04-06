# Tips for Flashing the Arduino Pro Mini

If you're having trouble with the "not in sync" error when flashing your Arduino Pro Mini, here are some techniques that often work:

## Timing the Reset

For Arduino Pro Mini boards, timing the reset is critical. Try this procedure:

1. Run the flash.ps1 script
2. When you see "Uploading to Arduino Pro Mini..." message, **immediately** press the reset button on your Arduino board
3. This synchronizes the bootloader with avrdude's upload attempt

## Hardware Setup

Ensure your connections are correct:
- If using an FTDI adapter, make sure:
  - TX from FTDI connects to RX on Arduino
  - RX from FTDI connects to TX on Arduino
  - DTR from FTDI connects to RST (Reset) on Arduino (if available)
  - Common ground (GND) between FTDI and Arduino
  - VCC supplying appropriate voltage (3.3V for 8MHz boards, 5V for 16MHz boards)

## Manual Reset Method

1. Edit the flash.ps1 script to add a pause:
   ```powershell
   Write-Host "Press the RESET button on your Arduino, then press Enter to continue..."
   Read-Host
   ```
2. Add this just before the avrdude command
3. When prompted, press the reset button on the Arduino, then quickly press Enter

## Try Different Programmer Types

If the above methods don't work, try changing the programmer type in the avrdude command:

```
# For USBasp programmers
avrdude -p atmega328p -c usbasp -P $env:AVR_COM_PORT -U flash:w:target\avr-atmega328p\release\blink.hex:i

# For AVRISP mkII
avrdude -p atmega328p -c avrispmkII -P $env:AVR_COM_PORT -U flash:w:target\avr-atmega328p\release\blink.hex:i

# For Arduino as ISP
avrdude -p atmega328p -c stk500v1 -P $env:AVR_COM_PORT -b 19200 -U flash:w:target\avr-atmega328p\release\blink.hex:i
```

## How to Confirm Success

When successfully flashed with our Rust program, the LED on pin 13 (B5) will blink the Morse code for "RUST":
- R: .-. (dot dash dot)
- U: ..- (dot dot dash)
- S: ... (dot dot dot)
- T: - (dash)

This is noticeably different from the default Arduino blink program, which typically has a much slower pace (around 1 second on, 1 second off). 