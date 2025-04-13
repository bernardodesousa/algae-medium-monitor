# 4-Digit 7-Segment Display Wiring Guide

## Components Required
- Arduino Pro Mini
- pH controller
- 7-segment common cathode 4-digit display
- 74HC595 shift register IC
- 4x 220 ohm resistors
- 1x 0.1μF ceramic capacitor (optional but recommended)
- 1x 10kΩ pull-up resistor (optional but recommended)
- Prototyping PCB
- Solid core wires in different colors

## Pin Assignments
- Arduino Pro Mini:
  - Pin B0 (D8) → 74HC595 DS (pin 8)
  - Pin B1 (D9) → 74HC595 STCP (pin 9)
  - Pin B2 (D10) → 74HC595 SHCP (pin 10)
  - 5V → 74HC595 VCC (pin 16)
  - GND → 74HC595 GND (pin 8)

- 74HC595 Shift Register:
  - VCC (pin 16) → 5V
  - GND (pin 8) → GND
  - STCP (pin 9) → Arduino B1
  - SHCP (pin 10) → Arduino B2
  - DS (pin 8) → Arduino B0
  - OE (pin 13) → GND
  - MR (pin 15) → 5V (or through 10kΩ pull-up resistor)
  - Q0-Q7 (pins 1-7, 15) → 7-segment display segments through 220Ω resistors

- 7-Segment Display (Common Cathode):
  - Common cathode pins → GND
  - Segment pins → 74HC595 outputs through 220Ω resistors

## Wiring Diagram (ASCII Art)
```
Arduino Pro Mini
+------------------+
|                  |
|  B0 (D8)  ------>|----> 74HC595 DS (pin 8)
|  B1 (D9)  ------>|----> 74HC595 STCP (pin 9)
|  B2 (D10) ------>|----> 74HC595 SHCP (pin 10)
|  5V       ------>|----> 74HC595 VCC (pin 16)
|  GND      ------>|----> 74HC595 GND (pin 8)
|                  |
+------------------+

74HC595 Shift Register
+------------------+
|                  |
|  Q0 (pin 1)  --->|----> 220Ω ---> 7-segment a
|  Q1 (pin 2)  --->|----> 220Ω ---> 7-segment b
|  Q2 (pin 3)  --->|----> 220Ω ---> 7-segment c
|  Q3 (pin 4)  --->|----> 220Ω ---> 7-segment d
|  Q4 (pin 5)  --->|----> 220Ω ---> 7-segment e
|  Q5 (pin 6)  --->|----> 220Ω ---> 7-segment f
|  Q6 (pin 7)  --->|----> 220Ω ---> 7-segment g
|  Q7 (pin 15) --->|----> 220Ω ---> 7-segment dp
|                  |
|  STCP (pin 9) <--|---- Arduino B1
|  SHCP (pin 10) <-|---- Arduino B2
|  DS (pin 8)   <--|---- Arduino B0
|  OE (pin 13)  -->|---- GND
|  MR (pin 15)  -->|---- 5V (or through 10kΩ)
|  VCC (pin 16) <--|---- 5V
|  GND (pin 8)  <--|---- GND
|                  |
+------------------+

7-Segment Display (Common Cathode)
+------------------+
|                  |
|  a  ------------<|---- 220Ω ---> 74HC595 Q0
|  b  ------------<|---- 220Ω ---> 74HC595 Q1
|  c  ------------<|---- 220Ω ---> 74HC595 Q2
|  d  ------------<|---- 220Ω ---> 74HC595 Q3
|  e  ------------<|---- 220Ω ---> 74HC595 Q4
|  f  ------------<|---- 220Ω ---> 74HC595 Q5
|  g  ------------<|---- 220Ω ---> 74HC595 Q6
|  dp ------------<|---- 220Ω ---> 74HC595 Q7
|                  |
|  Common Cathode -|---- GND
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
   - Add a 0.1μF ceramic capacitor between VCC and GND (close to the IC)

3. **Connect power and control lines**:
   - Connect 5V to pin 16 (VCC)
   - Connect GND to pin 8 (GND)
   - Connect OE (pin 13) to GND
   - Connect MR (pin 15) to 5V (or through 10kΩ resistor)
   - Connect Arduino control pins (B0, B1, B2) to the corresponding 74HC595 pins

4. **Connect the 7-segment display**:
   - Identify the common cathode pins (usually the middle pins)
   - Connect common cathode pins to GND
   - Connect each segment pin to the corresponding 74HC595 output through a 220Ω resistor

5. **Verify connections**:
   - Double-check all connections with a multimeter
   - Look for any solder bridges or cold joints
   - Ensure all components are properly oriented

## Troubleshooting Tips
- If the display doesn't light up, check the common cathode connection
- If some segments don't light up, check the corresponding resistor and connection
- If the display shows incorrect digits, check the segment-to-output mapping
- If nothing works, verify power connections and Arduino code

## Safety Considerations
- Use a temperature-controlled soldering iron set to 350-400°C
- Work in a well-ventilated area
- Avoid touching the hot soldering iron
- Double-check polarity of components before soldering
- Use heat shrink tubing or electrical tape to insulate exposed connections 