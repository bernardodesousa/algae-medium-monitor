use ruduino::Pin;
use ruduino::cores::current::port;

// Pin definitions for 74HC595
const LATCH_PIN: port::B1 = port::B1;  // STCP (pin 9)
const CLOCK_PIN: port::B2 = port::B2;  // SHCP (pin 10)
const DATA_PIN: port::B0 = port::B0;   // DS (pin 8)

// Pin definitions for digit selection (common cathode)
const DIGIT1_PIN: port::B3 = port::B3;  // Digit 1 common cathode
const DIGIT2_PIN: port::B4 = port::B4;  // Digit 2 common cathode
const DIGIT3_PIN: port::B6 = port::B6;  // Digit 3 common cathode
const DIGIT4_PIN: port::B7 = port::B7;  // Digit 4 common cathode

// Segment patterns for digits 0-9 and some letters
const DIGIT_PATTERNS: [u8; 17] = [
    0x3f, // 0
    0x06, // 1
    0x5b, // 2
    0x4f, // 3
    0x66, // 4
    0x6d, // 5
    0x7d, // 6
    0x07, // 7
    0x7f, // 8
    0x6f, // 9
    0x77, // A
    0x7c, // b
    0x39, // C
    0x5e, // d
    0x79, // E
    0x71, // F
    0x00  // blank
];

// Current display buffer
static mut DISPLAY_BUFFER: [u8; 4] = [0, 0, 0, 0];
static mut CURRENT_DIGIT: u8 = 0;

pub fn initialize() {
    // Set pins as outputs
    LATCH_PIN::set_output();
    CLOCK_PIN::set_output();
    DATA_PIN::set_output();
    DIGIT1_PIN::set_output();
    DIGIT2_PIN::set_output();
    DIGIT3_PIN::set_output();
    DIGIT4_PIN::set_output();
    
    // Initialize pins to low
    LATCH_PIN::set_low();
    CLOCK_PIN::set_low();
    DATA_PIN::set_low();
    
    // Initialize digit pins to high (inactive for common cathode)
    DIGIT1_PIN::set_high();
    DIGIT2_PIN::set_high();
    DIGIT3_PIN::set_high();
    DIGIT4_PIN::set_high();
    
    // Initialize display buffer
    unsafe {
        DISPLAY_BUFFER = [0, 0, 0, 0];
        CURRENT_DIGIT = 0;
    }
}

fn shift_out(data: u8) {
    // Shift out 8 bits
    for i in (0..8).rev() {
        // Set data bit
        if (data & (1 << i)) != 0 {
            DATA_PIN::set_high();
        } else {
            DATA_PIN::set_low();
        }
        
        // Clock pulse
        CLOCK_PIN::set_high();
        ruduino::delay::delay_ms(1);
        CLOCK_PIN::set_low();
        ruduino::delay::delay_ms(1);
    }
}

fn display_digit(digit: u8) {
    if digit > 16 {
        return; // Invalid digit
    }
    
    // Latch low to start
    LATCH_PIN::set_low();
    
    // Shift out the pattern
    shift_out(DIGIT_PATTERNS[digit as usize]);
    
    // Latch high to display
    LATCH_PIN::set_high();
}

// This function should be called regularly to update the display
pub fn update_display() {
    unsafe {
        // Turn off all digits
        DIGIT1_PIN::set_high();
        DIGIT2_PIN::set_high();
        DIGIT3_PIN::set_high();
        DIGIT4_PIN::set_high();
        
        // Display the current digit
        display_digit(DISPLAY_BUFFER[CURRENT_DIGIT as usize]);
        
        // Enable the current digit
        match CURRENT_DIGIT {
            0 => DIGIT1_PIN::set_low(),
            1 => DIGIT2_PIN::set_low(),
            2 => DIGIT3_PIN::set_low(),
            3 => DIGIT4_PIN::set_low(),
            _ => {}
        }
        
        // Move to next digit
        CURRENT_DIGIT = (CURRENT_DIGIT + 1) % 4;
    }
}

pub fn set_digit(position: u8, value: u8) {
    if position < 4 && value <= 16 {
        unsafe {
            DISPLAY_BUFFER[position as usize] = value;
        }
    }
}

pub fn display_number(num: u16) {
    // Convert number to individual digits
    let digits = [
        ((num / 1000) % 10) as u8,
        ((num / 100) % 10) as u8,
        ((num / 10) % 10) as u8,
        (num % 10) as u8
    ];
    
    // Update display buffer
    for i in 0..4 {
        set_digit(i, digits[i]);
    }
}

pub fn display_ph(ph: i16) {
    // Convert pH value to displayable format (multiply by 100 to show 2 decimal places)
    let display_value = (ph * 100) as u16;
    display_number(display_value);
}

pub fn display_temperature(temp: i16) {
    // Convert temperature to displayable format (multiply by 10 to show 1 decimal place)
    let display_value = (temp * 10) as u16;
    display_number(display_value);
}

// Function to display alternating values
pub fn display_alternating(ph: i16, temp: i16, counter: u8) {
    if counter % 2 == 0 {
        // Display pH
        display_ph(ph);
    } else {
        // Display temperature
        if let Some(temp) = temp {
            display_temperature(temp);
        } else {
            // Display "Err" if temperature sensor not detected
            set_digit(0, 14); // E
            set_digit(1, 16); // r
            set_digit(2, 16); // r
            set_digit(3, 0);  // blank
        }
    }
} 