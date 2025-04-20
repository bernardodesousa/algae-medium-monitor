# 4-Digit 7-Segment Display Pinout Guide

## 5643A Common Cathode 4-Digit Display Pinout

```
    a
   ---
f |   | b
   -g-
e |   | c
   ---
    d   dp

Pin Layout (Top View)
+------------------+
|                  |
|  1  2  3  4  5   |
|                  |
|  6  7  8  9  10  |
|                  |
|  11 12           |
|                  |
+------------------+

Pin Assignments:
1  - e
2  - d
3  - dp (decimal point)
4  - c
5  - g
6  - Common Cathode (Digit 4)
7  - b
8  - Common Cathode (Digit 3)
9  - Common Cathode (Digit 2)
10 - f
11 - a
12 - Common Cathode (Digit 1)
```

## Segment to 74HC595 Mapping

For a common cathode display, we need to set the corresponding bit HIGH to light up a segment:

```
74HC595 Output | 7-Segment Display Segment | Pin Number
----------------------------------------|------------
Q0 (pin 15)    | a                        | 11
Q1 (pin 1)     | b                        | 7
Q2 (pin 2)     | c                        | 4
Q3 (pin 3)     | d                        | 2
Q4 (pin 4)     | e                        | 1
Q5 (pin 5)     | f                        | 10
Q6 (pin 6)     | g                        | 5
Q7 (pin 7)     | dp (decimal point)       | 3
```

## Digit Patterns

The following byte patterns will display the corresponding digits on a common cathode display:

```
Digit | Binary  | Hex
------|---------|----
0     | 00111111| 0x3F
1     | 00000110| 0x06
2     | 01011011| 0x5B
3     | 01001111| 0x4F
4     | 01100110| 0x66
5     | 01101101| 0x6D
6     | 01111101| 0x7D
7     | 00000111| 0x07
8     | 01111111| 0x7F
9     | 01101111| 0x6F
A     | 01110111| 0x77
b     | 01111100| 0x7C
C     | 00111001| 0x39
d     | 01011110| 0x5E
E     | 01111001| 0x79
F     | 01110001| 0x71
blank | 00000000| 0x00
```

## Multiplexing Strategy

Since we're using a common cathode display, we need to multiplex the digits. The process is:

1. Set the segment pattern for the digit we want to display
2. Enable the corresponding common cathode pin (connect to GND)
3. Wait a short time (5ms)
4. Disable the common cathode pin
5. Repeat for the next digit

This creates the illusion that all digits are lit simultaneously due to persistence of vision.

## Common Cathode Pins

For the 5643A display, the common cathode pins are:
- Digit 1: Pin 12
- Digit 2: Pin 9
- Digit 3: Pin 8
- Digit 4: Pin 6

When multiplexing, you'll need to connect these pins to GND one at a time to activate each digit. 