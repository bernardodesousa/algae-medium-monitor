use ruduino::Pin;
use ruduino::cores::current::port;

// Temperature sensor pin
pub type DS18B20Pin = port::D2;  // T2 pin for Dallas one-wire temperature sensor

// Dallas one-wire commands
pub const SKIP_ROM: u8 = 0xCC;
pub const CONVERT_T: u8 = 0x44;
pub const READ_SCRATCHPAD: u8 = 0xBE;

// DS18B20 temp conversion time (worst case)
pub const TEMP_CONVERSION_TIME_MS: u32 = 750; // 12-bit resolution

// Initialize the DS18B20 pin
pub fn initialize() {
    DS18B20Pin::set_input();
    DS18B20Pin::set_high(); // Enable internal pull-up
}

// Reset the 1-Wire bus and check for device presence
pub fn reset() -> bool {
    // Pull the bus low for at least 480µs
    DS18B20Pin::set_output();
    DS18B20Pin::set_low();
    ruduino::delay::delay_us(500);
    
    // Release the bus and wait for the presence pulse
    DS18B20Pin::set_input();
    DS18B20Pin::set_high(); // Enable pull-up
    ruduino::delay::delay_us(70);
    
    // Read the bus state (low = device present)
    let device_present = !DS18B20Pin::is_high();
    
    // Wait for the reset sequence to finish
    ruduino::delay::delay_us(410);
    
    device_present
}

// Write a byte to the 1-Wire bus
pub fn write_byte(mut byte: u8) {
    for _ in 0..8 {
        // Extract the least significant bit
        let bit = byte & 0x01;
        byte >>= 1;
        
        DS18B20Pin::set_output();
        
        if bit == 0 {
            // Write 0: Pull low for 60-120µs
            DS18B20Pin::set_low();
            ruduino::delay::delay_us(70);
            DS18B20Pin::set_high();
            ruduino::delay::delay_us(5);
        } else {
            // Write 1: Pull low for 1-15µs, then release
            DS18B20Pin::set_low();
            ruduino::delay::delay_us(10);
            DS18B20Pin::set_high();
            ruduino::delay::delay_us(55);
        }
    }
}

// Read a byte from the 1-Wire bus
pub fn read_byte() -> u8 {
    let mut byte: u8 = 0;
    
    for i in 0..8 {
        DS18B20Pin::set_output();
        
        // Initiate read time slot with a low pulse
        DS18B20Pin::set_low();
        ruduino::delay::delay_us(5);
        
        // Release the bus
        DS18B20Pin::set_input();
        DS18B20Pin::set_high(); // Enable pull-up
        ruduino::delay::delay_us(10);
        
        // Read the bit value
        let bit = if DS18B20Pin::is_high() { 1 } else { 0 };
        byte |= bit << i;
        
        // Wait for time slot to complete
        ruduino::delay::delay_us(45);
    }
    
    byte
}

// Start a temperature conversion without waiting
pub fn start_temperature_conversion() {
    if reset() {
        write_byte(SKIP_ROM);
        write_byte(CONVERT_T);
    }
}

// Read temperature value after conversion is complete (no conversion wait)
pub fn read_temperature_after_conversion() -> Option<i16> {
    if reset() {
        write_byte(SKIP_ROM);
        write_byte(READ_SCRATCHPAD);
        
        // Read first two bytes of scratchpad (temperature data)
        let temp_low = read_byte();
        let temp_high = read_byte();
        
        // Combine bytes for temperature (as a u16 first)
        let raw_temp_u16 = ((temp_high as u16) << 8) | (temp_low as u16);
        
        // Check if reading is valid (not 0x0000 or 0xFFFF)
        if raw_temp_u16 != 0 && raw_temp_u16 != 0xFFFF {
            // Convert to temperature in degrees Celsius * 10 for one decimal place
            let raw_temp = raw_temp_u16 as i16;
            let temp_value = (raw_temp * 10) / 16;
            return Some(temp_value);
        }
    }
    
    None // Return None if temperature reading failed
}
