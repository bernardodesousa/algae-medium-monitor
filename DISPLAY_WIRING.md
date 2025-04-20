# 4-Digit 7-Segment Display Wiring Guide

## Components Required
- Arduino Pro Mini
- pH controller
- 7-segment common cathode 4-digit display (5643A)
- 74HC595 shift register IC
- 8x 220 ohm resistors (one for each segment)
- 1x 0.1μF ceramic capacitor (optional but recommended)
- 1x 10kΩ pull-up resistor (optional but recommended)
- Prototyping PCB
- Solid core wires in different colors

## Pin Assignments

### Arduino Pro Mini
- Pin B0 (D8) → 74HC595 DS (pin 14)
- Pin B1 (D9) → 74HC595 ST_CP (pin 12)
- Pin B2 (D10) → 74HC595 SH_CP (pin 11)
- Pin B3 (D11) → 7-segment Common Cathode (digit 1, pin 12)
- Pin B4 (D12) → 7-segment Common Cathode (digit 2, pin 9)
- Pin B6 (D13) → 7-segment Common Cathode (digit 3, pin 8)
- Pin B7 (D7) → 7-segment Common Cathode (digit 4, pin 6)
- 5V → 74HC595 VCC (pin 16)
- GND → 74HC595 GND (pin 8)

### 74HC595 Shift Register
- VCC (pin 16) → 5V
- GND (pin 8) → GND (this is the only power ground connection needed)
- ST_CP (pin 12) → Arduino B1
- SH_CP (pin 11) → Arduino B2
- DS (pin 14) → Arduino B0
- OE (pin 13) → GND (this is a control pin, not a power ground)
- MR (pin 10) → 5V (or through 10kΩ pull-up resistor)
- Q0 (pin 15) → 7-segment a through 220Ω resistor
- Q1 (pin 1) → 7-segment b through 220Ω resistor
- Q2 (pin 2) → 7-segment c through 220Ω resistor
- Q3 (pin 3) → 7-segment d through 220Ω resistor
- Q4 (pin 4) → 7-segment e through 220Ω resistor
- Q5 (pin 5) → 7-segment f through 220Ω resistor
- Q6 (pin 6) → 7-segment g through 220Ω resistor
- Q7 (pin 7) → 7-segment dp through 220Ω resistor
- Q7' (pin 9) → Not connected (used for cascading multiple shift registers)

### 7-Segment Display (5643A Common Cathode)
- Segment a (pin 11) → 74HC595 Q0 (pin 15) through 220Ω resistor
- Segment b (pin 7) → 74HC595 Q1 (pin 1) through 220Ω resistor
- Segment c (pin 4) → 74HC595 Q2 (pin 2) through 220Ω resistor
- Segment d (pin 2) → 74HC595 Q3 (pin 3) through 220Ω resistor
- Segment e (pin 1) → 74HC595 Q4 (pin 4) through 220Ω resistor
- Segment f (pin 10) → 74HC595 Q5 (pin 5) through 220Ω resistor
- Segment g (pin 5) → 74HC595 Q6 (pin 6) through 220Ω resistor
- Segment dp (pin 3) → 74HC595 Q7 (pin 7) through 220Ω resistor
- Common cathode (digit 1, pin 12) → Arduino B3
- Common cathode (digit 2, pin 9) → Arduino B4
- Common cathode (digit 3, pin 8) → Arduino B6
- Common cathode (digit 4, pin 6) → Arduino B7

## Wiring Diagram (ASCII Art)

```
Arduino Pro Mini
+------------------+
|                  |
|  B0 (D8)  ------>|----> 74HC595 DS (pin 14)
|  B1 (D9)  ------>|----> 74HC595 ST_CP (pin 12)
|  B2 (D10) ------>|----> 74HC595 SH_CP (pin 11)
|  B3 (D11) ------>|----> 7-segment CC Digit 1 (pin 12)
|  B4 (D12) ------>|----> 7-segment CC Digit 2 (pin 9)
|  B6 (D13) ------>|----> 7-segment CC Digit 3 (pin 8)
|  B7 (D7)  ------>|----> 7-segment CC Digit 4 (pin 6)
|  5V       ------>|----> 74HC595 VCC (pin 16)
|  GND      ------>|----> 74HC595 GND (pin 8)
|                  |
+------------------+

74HC595 Shift Register
+------------------+
|                  |
|  Q0 (pin 15) --->|----> 220Ω ---> 7-segment a (pin 11)
|  Q1 (pin 1)  --->|----> 220Ω ---> 7-segment b (pin 7)
|  Q2 (pin 2)  --->|----> 220Ω ---> 7-segment c (pin 4)
|  Q3 (pin 3)  --->|----> 220Ω ---> 7-segment d (pin 2)
|  Q4 (pin 4)  --->|----> 220Ω ---> 7-segment e (pin 1)
|  Q5 (pin 5)  --->|----> 220Ω ---> 7-segment f (pin 10)
|  Q6 (pin 6)  --->|----> 220Ω ---> 7-segment g (pin 5)
|  Q7 (pin 7)  --->|----> 220Ω ---> 7-segment dp (pin 3)
|                  |
|  ST_CP (pin 12) <--|---- Arduino B1
|  SH_CP (pin 11) <-|---- Arduino B2
|  DS (pin 14)   <--|---- Arduino B0
|  OE (pin 13)  -->|---- GND
|  MR (pin 10)  -->|---- 5V (or through 10kΩ)
|  VCC (pin 16) <--|---- 5V
|  GND (pin 8)  <--|---- GND
|                  |
+------------------+

7-Segment Display (5643A Common Cathode)
+------------------+
|                  |
|  a (pin 11) ----<|---- 220Ω ---> 74HC595 Q0 (pin 15)
|  b (pin 7)  ----<|---- 220Ω ---> 74HC595 Q1 (pin 1)
|  c (pin 4)  ----<|---- 220Ω ---> 74HC595 Q2 (pin 2)
|  d (pin 2)  ----<|---- 220Ω ---> 74HC595 Q3 (pin 3)
|  e (pin 1)  ----<|---- 220Ω ---> 74HC595 Q4 (pin 4)
|  f (pin 10) ----<|---- 220Ω ---> 74HC595 Q5 (pin 5)
|  g (pin 5)  ----<|---- 220Ω ---> 74HC595 Q6 (pin 6)
|  dp (pin 3) ----<|---- 220Ω ---> 74HC595 Q7 (pin 7)
|                  |
|  Common Cathode (digit 1, pin 12) --|---- Arduino B3
|  Common Cathode (digit 2, pin 9) --|---- Arduino B4
|  Common Cathode (digit 3, pin 8) --|---- Arduino B6
|  Common Cathode (digit 4, pin 6) --|---- Arduino B7
|                  |
+------------------+
```

## Assembly Steps

1. **Prepare the PCB**:
   - Clean the PCB with isopropyl alcohol
   - Plan your component placement to minimize wire crossings

2. **Solder the 74HC595**:
   - Place the IC in the center of your PCB
   - Solder all pins
   - Add a 0.1μF ceramic capacitor between VCC (pin 16) and GND (pin 8)
   - Place the capacitor as close as possible to these pins for proper power supply decoupling

3. **Connect power and control lines**:
   - Connect 5V to pin 16 (VCC)
   - Connect GND to pin 8 (GND) - this is the power ground connection
   - Connect OE (pin 13) to GND - this enables the outputs
   - Connect MR (pin 10) to 5V (or through 10kΩ resistor)
   - Connect Arduino control pins (B0, B1, B2) to the corresponding 74HC595 pins
   - Leave pin 9 (Q7') unconnected unless you plan to cascade multiple shift registers

4. **Connect the 7-segment display**:
   - Connect each common cathode pin to its own Arduino pin:
     - Pin 12 (digit 1) → Arduino B3
     - Pin 9 (digit 2) → Arduino B4
     - Pin 8 (digit 3) → Arduino B6
     - Pin 6 (digit 4) → Arduino B7
   - Connect each segment pin to the corresponding 74HC595 output through a 220Ω resistor:
     - Pin 11 (a) → Q0 (pin 15)
     - Pin 7 (b) → Q1 (pin 1)
     - Pin 4 (c) → Q2 (pin 2)
     - Pin 2 (d) → Q3 (pin 3)
     - Pin 1 (e) → Q4 (pin 4)
     - Pin 10 (f) → Q5 (pin 5)
     - Pin 5 (g) → Q6 (pin 6)
     - Pin 3 (dp) → Q7 (pin 7)

5. **Verify connections**:
   - Double-check all connections with a multimeter
   - Look for any solder bridges or cold joints
   - Ensure all components are properly oriented

## How Multiplexing Works

The multiplexing technique allows us to control all 32 segments (8 segments × 4 digits) using only 12 pins:
- 3 pins to control the shift register
- 4 pins to control which digit is active
- Plus power and ground

The process works as follows:
1. We send data to the shift register for digit 1
2. We enable only the common cathode for digit 1
3. We quickly move to digit 2, sending its data to the shift register
4. We enable only the common cathode for digit 2
5. And so on for digits 3 and 4
6. We repeat this process rapidly (>100 Hz) to create the illusion that all digits are lit simultaneously

## Important Multiplexing Tips
- **Never connect common cathodes directly to ground** - they must be controlled by Arduino pins
- Keep refresh rate high (low delay between updates) to avoid visible flickering
- Use short delays between digit transitions to ensure clean display
- When first testing, use simple patterns like displaying "1234" before attempting more complex displays

## Troubleshooting Tips
- If the display doesn't light up, check the common cathode connections
- If some segments don't light up, check the corresponding resistor and connection
- If the display shows incorrect digits, check the segment-to-output mapping
- If nothing works, verify power connections and Arduino code
- If you see flickering, adjust the refresh rate in your code to be faster
- If the same digit appears on all positions, verify that common cathodes are connected to Arduino pins and not to ground

## Safety Considerations
- Use a temperature-controlled soldering iron set to 350-400°C
- Work in a well-ventilated area
- Avoid touching the hot soldering iron
- Double-check polarity of components before soldering
- Use heat shrink tubing or electrical tape to insulate exposed connections 