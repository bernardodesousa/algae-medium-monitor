use ruduino::Pin;
use ruduino::cores::current::port::{B0, B1, B2, B3, B4, D3, D4};
use ruduino::delay;
use core::sync::atomic::{AtomicBool, Ordering};
use libm::roundf;
use core::arch::asm;

// Memory addresses for Timer0 registers
const TCCR0B: *mut u8 = 0x25 as *mut u8;  // Timer/Counter0 Control Register B
const TIMSK0: *mut u8 = 0x6E as *mut u8;  // Timer/Counter0 Interrupt Mask Register

// Current display buffer and decimal point buffer
static mut DISPLAY_BUFFER: [u8; 4] = [0, 0, 0, 0];
static mut DP_BUFFER: [bool; 4] = [false, false, false, false];
static mut CURRENT_DIGIT: u8 = 0;

// Disable auto-update by default for direct control
static AUTO_UPDATE_ENABLED: AtomicBool = AtomicBool::new(false);

// Constants for the different display patterns to show
const PATTERN_ALL_ON: u8 = 0xFF;  // All segments on
// const PATTERN_TEST: u8 = 0xD7;    // A unique test pattern

// Segment patterns for digits 0-9 and some letters
const DIGIT_PATTERNS: [u8; 17] = [
    0x3f, // 0: 0b00111111
    0x06, // 1: 0b00000110
    0x5b, // 2: 0b01011011
    0x4f, // 3: 0b01001111
    0x66, // 4: 0b01100110
    0x6d, // 5: 0b01101101
    0x7d, // 6: 0b01111101
    0x07, // 7: 0b00000111
    0x7f, // 8: 0b01111111
    0x6f, // 9: 0b01101111
    0x77, // A: 0b01110111
    0x7c, // b: 0b01111100
    0x39, // C: 0b00111001
    0x5e, // d: 0b01011110
    0x79, // E: 0b01111001
    0x71, // F: 0b01110001
    0x00  // blank: 0b00000000
];

/// Initialize the 7-segment display with auto-update capability
pub fn initialize() {
    // Set pins as outputs
    B1::set_output(); // LATCH_PIN (STCP)
    B2::set_output(); // CLOCK_PIN (SHCP)
    B0::set_output(); // DATA_PIN (DS)
    B3::set_output(); // DIGIT1_PIN
    B4::set_output(); // DIGIT2_PIN
    D3::set_output(); // DIGIT3_PIN
    D4::set_output(); // DIGIT4_PIN
    
    // Initialize pins to low
    B1::set_low(); // LATCH_PIN
    B2::set_low(); // CLOCK_PIN
    B0::set_low(); // DATA_PIN
    
    // Initialize digit pins to high (inactive for common cathode)
    B3::set_high(); // DIGIT1_PIN
    B4::set_high(); // DIGIT2_PIN
    D3::set_high(); // DIGIT3_PIN
    D4::set_high(); // DIGIT4_PIN
    
    // Add a brief delay after initialization
    delay::delay_ms(50);
    
    // Initialize display buffer
    unsafe {
        DISPLAY_BUFFER = [8, 8, 8, 8]; // Display all 8's as default
        DP_BUFFER = [false, false, false, false];
        CURRENT_DIGIT = 0;
        
        // Enable Timer0 overflow interrupt
        *TIMSK0 |= 1 << 0;
        
        // Set Timer0 prescaler to 1024
        *TCCR0B = (*TCCR0B & !0x07) | 0x05;
        
        // Enable global interrupts
        asm!("sei");
    }
    
    // Do not auto-update by default - we'll use manual updates for testing
    AUTO_UPDATE_ENABLED.store(false, Ordering::Relaxed);
    
    // Test each digit individually for debugging
    test_all_segments();
}

// Test function that manually tests all display segments
fn test_all_segments() {
    // Test all digits with all segments lit
    for digit in 0..4 {
        // Turn off all digits
        B3::set_high();
        B4::set_high();
        D3::set_high();
        D4::set_high();
        
        // Send all segments on pattern
        shift_out(PATTERN_ALL_ON);
        
        // Enable current digit only
        match digit {
            0 => B3::set_low(), // DIGIT1_PIN
            1 => B4::set_low(), // DIGIT2_PIN
            2 => D3::set_low(), // DIGIT3_PIN
            3 => D4::set_low(), // DIGIT4_PIN
            _ => {}
        }
        
        // Hold for a visible time
        delay::delay_ms(200);
    }
    
    // Turn off all digits
    B3::set_high();
    B4::set_high();
    D3::set_high();
    D4::set_high();
}

// Shift out data to the 74HC595 with more robust timing
fn shift_out(data: u8) {
    // Set latch low before shifting
    B1::set_low(); // LATCH_PIN
    delay::delay_us(5);
    
    // Shift out 8 bits MSB first
    for i in (0..8).rev() {
        // Set data bit
        if (data & (1 << i)) != 0 {
            B0::set_high(); // DATA_PIN
        } else {
            B0::set_low(); // DATA_PIN
        }
        
        // Longer delay for setup time
        delay::delay_us(5);
        
        // Clock pulse with longer delays
        B2::set_high(); // CLOCK_PIN
        delay::delay_us(10);
        B2::set_low(); // CLOCK_PIN
        delay::delay_us(5);
    }
    
    // Set latch high to display with delay
    delay::delay_us(5);
    B1::set_high(); // LATCH_PIN
    delay::delay_us(10);
}

// Display a single digit with optional decimal point
fn display_digit(digit: u8, show_dp: bool) {
    if digit > 16 {
        return; // Invalid digit
    }
    
    let mut pattern = DIGIT_PATTERNS[digit as usize];
    
    // Add decimal point if needed (bit 7 controls DP segment)
    if show_dp {
        pattern |= 0x80; // Set the DP bit (bit 7)
    }
    
    shift_out(pattern);
}

/// Timer0 overflow interrupt handler for auto-updating the display
#[no_mangle]
pub extern "avr-interrupt" fn __vector_16() {
    if AUTO_UPDATE_ENABLED.load(Ordering::Relaxed) {
        update_display_internal();
    }
}

/// Internal function to update the display (used by timer interrupt)
fn update_display_internal() {
    unsafe {
        // Turn off all digits first
        B3::set_high(); // DIGIT1_PIN
        B4::set_high(); // DIGIT2_PIN
        D3::set_high(); // DIGIT3_PIN
        D4::set_high(); // DIGIT4_PIN
        
        // Display the digit for the current position with its decimal point
        display_digit(DISPLAY_BUFFER[CURRENT_DIGIT as usize], DP_BUFFER[CURRENT_DIGIT as usize]);
        
        // Enable only the current digit position
        match CURRENT_DIGIT {
            0 => B3::set_low(), // DIGIT1_PIN
            1 => B4::set_low(), // DIGIT2_PIN
            2 => D3::set_low(), // DIGIT3_PIN
            3 => D4::set_low(), // DIGIT4_PIN
            _ => {}
        }
        
        // Move to next digit position
        CURRENT_DIGIT = (CURRENT_DIGIT + 1) % 4;
    }
}

/// Update the display manually (only needed if auto-update is disabled)
/// This function is kept for backward compatibility but is not needed in normal operation
pub fn update() {
    if !AUTO_UPDATE_ENABLED.load(Ordering::Relaxed) {
        update_display_internal();
    }
}

/// Enable or disable automatic display updates
pub fn set_auto_update(enabled: bool) {
    AUTO_UPDATE_ENABLED.store(enabled, Ordering::Relaxed);
}

// Set a specific digit value and its decimal point
fn set_digit(position: u8, value: u8, decimal_point: bool) {
    if position < 4 && value <= 16 {
        unsafe {
            DISPLAY_BUFFER[position as usize] = value;
            DP_BUFFER[position as usize] = decimal_point;
        }
    }
}

/// Display a floating point number on the 7-segment display
/// 
/// The function automatically formats the number based on its magnitude:
/// - Numbers < 10: Format as #.### (e.g., 1.234)
/// - Numbers 10-99: Format as ##.## (e.g., 12.34)
/// - Numbers 100-999: Format as ###.# (e.g., 123.4)
/// - Numbers 1000-9999: Format as #### (e.g., 1234)
///
/// Numbers are automatically rounded to fit the display format.
pub fn display(mut num: f32) {
    // Check if number is in valid range (0 to 9999.9999)
    if num < 0.0 || num > 9999.9999 {
        // Display "Err" for out of range
        set_digit(0, 14, false); // E
        set_digit(1, 16, false); // r
        set_digit(2, 16, false); // r
        set_digit(3, 16, false); // blank
        return;
    }
    
    // Format based on magnitude
    if num < 10.0 {
        // Format as #.###
        num = roundf(num * 1000.0) / 1000.0;
        
        let d1 = (num as u16) % 10;
        let d2 = ((num * 10.0) as u16) % 10;
        let d3 = ((num * 100.0) as u16) % 10;
        let d4 = ((num * 1000.0) as u16) % 10;
        
        set_digit(0, d1 as u8, true); // First digit with DP
        set_digit(1, d2 as u8, false);
        set_digit(2, d3 as u8, false);
        set_digit(3, d4 as u8, false);
    } else if num < 100.0 {
        // Format as ##.##
        num = roundf(num * 100.0) / 100.0;
        
        let d1 = ((num / 10.0) as u16) % 10;
        let d2 = (num as u16) % 10;
        let d3 = ((num * 10.0) as u16) % 10;
        let d4 = ((num * 100.0) as u16) % 10;
        
        set_digit(0, d1 as u8, false);
        set_digit(1, d2 as u8, true); // Second digit with DP
        set_digit(2, d3 as u8, false);
        set_digit(3, d4 as u8, false);
    } else if num < 1000.0 {
        // Format as ###.#
        num = roundf(num * 10.0) / 10.0;
        
        let d1 = ((num / 100.0) as u16) % 10;
        let d2 = ((num / 10.0) as u16) % 10;
        let d3 = (num as u16) % 10;
        let d4 = ((num * 10.0) as u16) % 10;
        
        set_digit(0, d1 as u8, false);
        set_digit(1, d2 as u8, false);
        set_digit(2, d3 as u8, true); // Third digit with DP
        set_digit(3, d4 as u8, false);
    } else {
        // Format as ####
        num = roundf(num);
        
        let d1 = ((num / 1000.0) as u16) % 10;
        let d2 = ((num / 100.0) as u16) % 10;
        let d3 = ((num / 10.0) as u16) % 10;
        let d4 = (num as u16) % 10;
        
        set_digit(0, d1 as u8, false);
        set_digit(1, d2 as u8, false);
        set_digit(2, d3 as u8, false);
        set_digit(3, d4 as u8, false);
    }
}
