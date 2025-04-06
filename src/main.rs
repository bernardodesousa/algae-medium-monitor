#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};

// Morse code timings
const DOT_MS: u32 = 200;     // Short blink
const DASH_MS: u32 = 600;    // Long blink
const SYMBOL_SPACE_MS: u32 = 200;  // Space between dots and dashes
const LETTER_SPACE_MS: u32 = 600;  // Space between letters
const WORD_SPACE_MS: u32 = 1400;   // Space between words

#[no_mangle]
pub extern "C" fn main() {
    port::B5::set_output();

    loop {
        // "RUST" in Morse code
        
        // R: .-.
        blink_dot();
        blink_dash();
        blink_dot();
        ruduino::delay::delay_ms(LETTER_SPACE_MS);
        
        // U: ..-
        blink_dot();
        blink_dot();
        blink_dash();
        ruduino::delay::delay_ms(LETTER_SPACE_MS);
        
        // S: ...
        blink_dot();
        blink_dot();
        blink_dot();
        ruduino::delay::delay_ms(LETTER_SPACE_MS);
        
        // T: -
        blink_dash();
        
        // Pause before repeating
        ruduino::delay::delay_ms(WORD_SPACE_MS);
    }
}

// Helper functions for Morse code
fn blink_dot() {
    port::B5::set_high();
    ruduino::delay::delay_ms(DOT_MS);
    port::B5::set_low();
    ruduino::delay::delay_ms(SYMBOL_SPACE_MS);
}

fn blink_dash() {
    port::B5::set_high();
    ruduino::delay::delay_ms(DASH_MS);
    port::B5::set_low();
    ruduino::delay::delay_ms(SYMBOL_SPACE_MS);
}
