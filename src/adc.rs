use ruduino::Register;

// Register definitions for ADC
pub struct ADCSRA;
impl Register for ADCSRA {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7A as *mut u8;
}

// ADC Multiplexer Selection Register
pub struct ADMUX;
impl Register for ADMUX {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7C as *mut u8;
}

// ADC Data Register High Byte
pub struct ADCH;
impl Register for ADCH {
    type T = u8;
    const ADDRESS: *mut u8 = 0x79 as *mut u8;
}

// ADC Data Register Low Byte
pub struct ADCL;
impl Register for ADCL {
    type T = u8;
    const ADDRESS: *mut u8 = 0x78 as *mut u8;
}

// ADCSRA bits
pub const ADEN: u8 = 1 << 7;   // ADC Enable
pub const ADSC: u8 = 1 << 6;   // ADC Start Conversion
pub const ADIF: u8 = 1 << 4;   // ADC Interrupt Flag
pub const ADIE: u8 = 1 << 3;   // ADC Interrupt Enable
pub const ADPS2: u8 = 1 << 2;  // ADC Prescaler Select Bit 2
pub const ADPS1: u8 = 1 << 1;  // ADC Prescaler Select Bit 1
pub const ADPS0: u8 = 1 << 0;  // ADC Prescaler Select Bit 0

// ADMUX bits
pub const REFS1: u8 = 1 << 7;  // Reference Selection Bit 1
pub const REFS0: u8 = 1 << 6;  // Reference Selection Bit 0
pub const ADLAR: u8 = 1 << 5;  // ADC Left Adjust Result
pub const MUX3: u8 = 1 << 3;   // Analog Channel Selection Bit 3
pub const MUX2: u8 = 1 << 2;   // Analog Channel Selection Bit 2
pub const MUX1: u8 = 1 << 1;   // Analog Channel Selection Bit 1
pub const MUX0: u8 = 1 << 0;   // Analog Channel Selection Bit 0

// ADC channel constants
pub const ADC0: u8 = 0;  // For pH sensor (Po)
pub const ADC1: u8 = 1;  // For T1 analog temperature

// Initialize the ADC
pub fn initialize() {
    // Set reference voltage to AVCC with external capacitor at AREF pin
    ADMUX::write(REFS0); 
    
    // Enable ADC and set prescaler to 128 (16MHz/128 = 125KHz)
    // ADC requires an input clock frequency between 50KHz and 200KHz for maximum resolution
    ADCSRA::write(ADEN | ADPS2 | ADPS1 | ADPS0);
}

// Read from the specified ADC channel
pub fn read(channel: u8) -> u16 {
    // Select ADC channel with safety mask (0x07 ensures we only affect the MUX bits)
    let admux = ADMUX::read() & 0xF0; // Clear the lower 4 bits for channel selection
    ADMUX::write(admux | (channel & 0x07));
    
    // Start ADC conversion
    let adcsra = ADCSRA::read();
    ADCSRA::write(adcsra | ADSC);
    
    // Wait for conversion to complete
    while ADCSRA::read() & ADSC != 0 {}
    
    // Read ADC result - first low byte, then high byte
    let low = ADCL::read();
    let high = ADCH::read();
    
    // Combine the two bytes
    ((high as u16) << 8) | (low as u16)
}
