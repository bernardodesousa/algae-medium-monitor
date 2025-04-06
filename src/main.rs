#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::port;
use ruduino::Register;

// Pins for the sensors
const ADC0: u8 = 0;  // For pH sensor

// DS18B20 temperature sensor on digital pin D2 (PD2)
// Connected through the pH controller board
// Typically: Yellow/White (data) -> D2, Red -> 5V, Black -> GND

// Use the built-in D2 pin definition for the DS18B20
// On ATmega328P, D2 is PD2 (Port D, bit 2)
type DS18B20Pin = port::D2;

// UART configuration
const BAUD_RATE: u32 = 9600;
const CPU_FREQUENCY: u32 = 16_000_000;
const UBRR_VALUE: u16 = (CPU_FREQUENCY / (16 * BAUD_RATE) - 1) as u16;

// DS18B20 ROM commands
const SKIP_ROM: u8 = 0xCC;
const CONVERT_T: u8 = 0x44;
const READ_SCRATCHPAD: u8 = 0xBE;

// DS18B20 temperature conversion times (worst case)
const TEMP_CONVERSION_TIME_MS: u64 = 750; // 12-bit resolution takes max 750ms

// Timing constants
const READING_INTERVAL_MS: u32 = 2000; // Time between readings
const RETRY_COUNT: u8 = 3; // Number of retry attempts for DS18B20

// Register definitions for ADC
mod adc {
    use ruduino::Register;
    
    // ADC Control and Status Register A
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
}

// UART Register definitions
mod uart {
    use ruduino::Register;
    
    // USART Control and Status Register A
    pub struct UCSR0A;
    impl Register for UCSR0A {
        type T = u8;
        const ADDRESS: *mut u8 = 0xC0 as *mut u8;
    }
    
    // USART Control and Status Register B
    pub struct UCSR0B;
    impl Register for UCSR0B {
        type T = u8;
        const ADDRESS: *mut u8 = 0xC1 as *mut u8;
    }
    
    // USART Control and Status Register C
    pub struct UCSR0C;
    impl Register for UCSR0C {
        type T = u8;
        const ADDRESS: *mut u8 = 0xC2 as *mut u8;
    }
    
    // USART Baud Rate Register Low
    pub struct UBRR0L;
    impl Register for UBRR0L {
        type T = u8;
        const ADDRESS: *mut u8 = 0xC4 as *mut u8;
    }
    
    // USART Baud Rate Register High
    pub struct UBRR0H;
    impl Register for UBRR0H {
        type T = u8;
        const ADDRESS: *mut u8 = 0xC5 as *mut u8;
    }
    
    // USART Data Register
    pub struct UDR0;
    impl Register for UDR0 {
        type T = u8;
        const ADDRESS: *mut u8 = 0xC6 as *mut u8;
    }
    
    // UCSR0A bits
    pub const RXC0: u8 = 1 << 7;   // USART Receive Complete
    pub const TXC0: u8 = 1 << 6;   // USART Transmit Complete
    pub const UDRE0: u8 = 1 << 5;  // USART Data Register Empty
    
    // UCSR0B bits
    pub const RXEN0: u8 = 1 << 4;  // Receiver Enable
    pub const TXEN0: u8 = 1 << 3;  // Transmitter Enable
    
    // UCSR0C bits
    pub const UCSZ01: u8 = 1 << 2; // Character Size bit 1
    pub const UCSZ00: u8 = 1 << 1; // Character Size bit 0
}

#[no_mangle]
pub extern "C" fn main() {
    // Initialize ADC for pH sensor
    initialize_adc();
    
    // Initialize UART
    initialize_uart();

    // Status LED
    port::B5::set_output();
    
    // Configure DS18B20 pin (initially as input)
    DS18B20Pin::set_input();
    DS18B20Pin::set_high(); // Enable internal pull-up
    
    // Temperature and pH values
    let mut temp_value: i16 = 0;
    let mut ph_value: u16;
    
    // Variables for tracking DS18B20 readings
    let mut temp_raw_low: u8 = 0;
    let mut temp_raw_high: u8 = 0;
    let mut ds18b20_found = false;

    // Send startup message
    uart_send_string("Algae Medium Monitor Starting...\r\n");
    uart_send_string("DS18B20 via pH controller board\r\n");

    loop {
        // Blink LED to indicate cycle start
        port::B5::set_high();
        
        // Read temperature from DS18B20 with retries
        ds18b20_found = false;
        
        for _ in 0..RETRY_COUNT {
            if ds18b20_reset() {
                ds18b20_found = true;
                
                // Start temperature conversion
                ds18b20_write_byte(SKIP_ROM);
                ds18b20_write_byte(CONVERT_T);
                
                // Wait for conversion to complete
                ruduino::delay::delay_ms(TEMP_CONVERSION_TIME_MS);
                
                // Read temperature data
                if ds18b20_reset() {
                    ds18b20_write_byte(SKIP_ROM);
                    ds18b20_write_byte(READ_SCRATCHPAD);
                    
                    // Read first two bytes of scratchpad (temperature data)
                    temp_raw_low = ds18b20_read_byte();
                    temp_raw_high = ds18b20_read_byte();
                    
                    // Combine bytes for temperature (as a u16 first)
                    let raw_temp_u16 = ((temp_raw_high as u16) << 8) | (temp_raw_low as u16);
                    
                    // Check if reading is valid (not 0x0000 or 0xFFFF)
                    if raw_temp_u16 != 0 && raw_temp_u16 != 0xFFFF {
                        // Convert to temperature in degrees Celsius * 10 for one decimal place
                        // First convert to i16 for signed temperature support
                        let raw_temp = raw_temp_u16 as i16;
                        // (Raw value is 1/16 degrees Celsius)
                        temp_value = (raw_temp * 10) / 16;
                        break; // Valid reading, exit retry loop
                    }
                }
            }
            
            // If we got here in the retry loop, try again after a short delay
            ruduino::delay::delay_ms(100);
        }
        
        // Read pH value from analog pin A0 (PC0)
        let ph_raw = read_adc(ADC0);
        
        // Convert pH reading (simplified conversion)
        // Assuming pH range 0-14 maps to ADC range 0-1023
        // We'll multiply by 100 to get two decimal places
        ph_value = ((ph_raw as u32 * 1400) / 1023) as u16;
        
        // Finish LED blink
        port::B5::set_low();
        
        // Send data over UART
        uart_send_string("DS18B20: ");
        if ds18b20_found {
            uart_send_string("Found - T:");
            uart_send_temperature(temp_value);
        } else {
            uart_send_string("Not found");
            temp_value = -9990; // Error value
        }
        
        uart_send_string(" pH:");
        uart_send_ph(ph_value);
        uart_send_string("\r\n");
        
        // Send raw values for debugging
        uart_send_string("Raw values - T: 0x");
        if ds18b20_found {
            uart_send_integer(temp_raw_low as u16, 16);
            uart_send_string(" 0x");
            uart_send_integer(temp_raw_high as u16, 16);
        } else {
            uart_send_string("?? ??");
        }
        uart_send_string(" pH: ");
        uart_send_integer(ph_raw, 10);
        uart_send_string("\r\n");
        
        // Wait before next reading
        ruduino::delay::delay_ms(READING_INTERVAL_MS.into());
    }
}

// DS18B20 1-Wire Protocol Functions

// Reset the 1-Wire bus and check for device presence
// Returns true if a device is present, false otherwise
fn ds18b20_reset() -> bool {
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
fn ds18b20_write_byte(mut byte: u8) {
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
fn ds18b20_read_byte() -> u8 {
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

// Initialize the UART
fn initialize_uart() {
    // Set baud rate
    let ubrr = UBRR_VALUE;
    uart::UBRR0H::write((ubrr >> 8) as u8);
    uart::UBRR0L::write(ubrr as u8);
    
    // Enable transmitter
    uart::UCSR0B::write(uart::TXEN0);
    
    // Set frame format: 8 data bits, 1 stop bit, no parity
    uart::UCSR0C::write(uart::UCSZ01 | uart::UCSZ00);
}

// Send a single byte over UART
fn uart_send_byte(data: u8) {
    // Wait for the transmit buffer to be empty
    while uart::UCSR0A::read() & uart::UDRE0 == 0 {}
    
    // Send the data
    uart::UDR0::write(data);
}

// Send a string over UART
fn uart_send_string(s: &str) {
    for byte in s.bytes() {
        uart_send_byte(byte);
    }
}

// Send a temperature value over UART (format: XX.X)
fn uart_send_temperature(value: i16) {
    // Check for error value
    if value == -9990 {
        uart_send_string("Error");
        return;
    }
    
    // Handle negative temperatures
    if value < 0 {
        uart_send_byte(b'-');
        uart_send_decimal(-value as u16, 1);
    } else {
        uart_send_decimal(value as u16, 1);
    }
}

// Send a pH value over UART (format: X.XX)
fn uart_send_ph(value: u16) {
    uart_send_decimal(value, 2);
}

// Send a decimal number with specified number of decimal places
fn uart_send_decimal(value: u16, decimal_places: u8) {
    let mut divisor = 1;
    for _ in 0..decimal_places {
        divisor *= 10;
    }
    
    let whole = value / divisor;
    let fraction = value % divisor;
    
    // Send whole part
    uart_send_integer(whole, 10);
    
    // Send decimal point
    uart_send_byte(b'.');
    
    // Send fractional part with leading zeros if needed
    let mut fraction_divisor = divisor / 10;
    while fraction_divisor > 0 {
        let digit = (fraction / fraction_divisor) % 10;
        uart_send_byte(b'0' + digit as u8);
        fraction_divisor /= 10;
    }
}

// Send an integer value with the specified base
fn uart_send_integer(mut value: u16, base: u16) {
    const BUFFER_SIZE: usize = 16;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut i = BUFFER_SIZE;
    
    // Handle the case of zero separately
    if value == 0 {
        uart_send_byte(b'0');
        return;
    }
    
    // Convert the number to ASCII in reverse order
    while value > 0 && i > 0 {
        i -= 1;
        let digit = (value % base) as u8;
        buffer[i] = if digit < 10 {
            b'0' + digit
        } else {
            b'A' + (digit - 10)
        };
        value /= base;
    }
    
    // Send the digits
    for j in i..BUFFER_SIZE {
        uart_send_byte(buffer[j]);
    }
}

// Initialize the ADC
fn initialize_adc() {
    // Set reference voltage to AVCC with external capacitor at AREF pin
    adc::ADMUX::write(adc::REFS0); 
    
    // Enable ADC and set prescaler to 128 (16MHz/128 = 125KHz)
    // ADC requires an input clock frequency between 50KHz and 200KHz for maximum resolution
    adc::ADCSRA::write(adc::ADEN | adc::ADPS2 | adc::ADPS1 | adc::ADPS0);
}

// Read from the specified ADC channel
fn read_adc(channel: u8) -> u16 {
    // Select ADC channel with safety mask (0x07 ensures we only affect the MUX bits)
    let admux = adc::ADMUX::read() & 0xF0; // Clear the lower 4 bits for channel selection
    adc::ADMUX::write(admux | (channel & 0x07));
    
    // Start ADC conversion
    let adcsra = adc::ADCSRA::read();
    adc::ADCSRA::write(adcsra | adc::ADSC);
    
    // Wait for conversion to complete
    while adc::ADCSRA::read() & adc::ADSC != 0 {}
    
    // Read ADC result - first low byte, then high byte
    let low = adc::ADCL::read();
    let high = adc::ADCH::read();
    
    // Combine the two bytes
    ((high as u16) << 8) | (low as u16)
}
