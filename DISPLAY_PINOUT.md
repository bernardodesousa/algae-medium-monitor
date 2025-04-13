# 4-Digit 7-Segment Display Pinout Guide

## Common Cathode 4-Digit Display Pinout

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
+------------------+

Pin Assignments:
1  - e
2  - d
3  - dp (decimal point)
4  - c
5  - g
6  - b
7  - a
8  - f
9  - Common Cathode (Digit 1)
10 - Common Cathode (Digit 2)
11 - Common Cathode (Digit 3)
12 - Common Cathode (Digit 4)
```

## Segment to 74HC595 Mapping

For a common cathode display, we need to set the corresponding bit HIGH to light up a segment:

```
74HC595 Output | 7-Segment Display Segment
----------------------------------------
Q0 (pin 1)     | a
Q1 (pin 2)     | b
Q2 (pin 3)     | c
Q3 (pin 4)     | d
Q4 (pin 5)     | e
Q5 (pin 6)     | f
Q6 (pin 7)     | g
Q7 (pin 15)    | dp (decimal point)
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