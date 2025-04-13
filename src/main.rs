#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::port;
use ruduino::Register;

// Pins for the sensors
const ADC0: u8 = 0;  // For pH sensor (Po)
const ADC1: u8 = 1;  // For T1 analog temperature (currently disconnected)
type DS18B20Pin = port::D2;  // T2 pin for Dallas one-wire temperature sensor

// Based on product description:
// Po: PH Value Output
// T1: Temperature Simulation Output
// T2: Temperature probe (Dallas one-wire protocol)

// Dallas one-wire commands
const SKIP_ROM: u8 = 0xCC;
const CONVERT_T: u8 = 0x44;
const READ_SCRATCHPAD: u8 = 0xBE;

// DS18B20 temp conversion time (worst case)
const TEMP_CONVERSION_TIME_MS: u32 = 750; // 12-bit resolution

// Retry attempts for temperature readings
const RETRY_COUNT: u8 = 3;

// UART configuration
const BAUD_RATE: u32 = 9600;
const CPU_FREQUENCY: u32 = 16_000_000;
const UBRR_VALUE: u16 = (CPU_FREQUENCY / (16 * BAUD_RATE) - 1) as u16;

// Timing constants
const READING_INTERVAL_MS: u32 = 2000; // Time between readings

// Calibration mode - set to true when calibrating
const CALIBRATION_MODE: bool = true;

// Known pH values for calibration
const PH_ACID: f32 = 2.2;  // Lime juice approximate pH
const PH_NEUTRAL: f32 = 7.0;  // Water approximate pH
const PH_ALKALINE: f32 = 8.4;  // Baking soda solution approximate pH

// pH conversion parameters
// These parameters map the ADC values to pH values
// Based on observations: Higher ADC = LOWER pH, Lower ADC = HIGHER pH
const PH_MIN_ADC: u16 = 1020;  // ADC value corresponding to pH MIN
const PH_MAX_ADC: u16 = 650;   // ADC value corresponding to pH MAX
const PH_MIN: u16 = 200;       // pH 2.00 * 100
const PH_MAX: u16 = 1400;      // pH 14.00 * 100

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
    
    // Configure DS18B20 pin
    DS18B20Pin::set_input();
    DS18B20Pin::set_high(); // Enable internal pull-up
    
    // Temperature and pH values
    let mut temp_value: i16 = 0;
    let mut temp_analog: i16 = 0;
    let mut ph_value: u16;
    let mut temp_raw_low: u8 = 0;
    let mut temp_raw_high: u8 = 0;
    
    // Send startup message
    uart_send_string("Algae Medium Monitor Starting...\r\n");
    uart_send_string("Reading sensors - HYBRID APPROACH:\r\n");
    uart_send_string("Po (ADC0): pH Value Output\r\n");
    uart_send_string("T1 (ADC1): Analog Temperature Output\r\n");
    uart_send_string("T2 (D2): Dallas One-Wire Temperature Sensor\r\n");

    loop {
        // Blink LED to indicate cycle start
        port::B5::set_high();
        
        // 1. Try to read temperature from DS18B20 (digital)
        let mut ds18b20_found = false;
        
        for _ in 0..RETRY_COUNT {
            if ds18b20_reset() {
                ds18b20_found = true;
                
                // Start temperature conversion
                ds18b20_write_byte(SKIP_ROM);
                ds18b20_write_byte(CONVERT_T);
                
                // Wait for conversion to complete
                ruduino::delay::delay_ms(TEMP_CONVERSION_TIME_MS.into());
                
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
                        let raw_temp = raw_temp_u16 as i16;
                        temp_value = (raw_temp * 10) / 16;
                        break; // Valid reading, exit retry loop
                    }
                }
            }
            ruduino::delay::delay_ms(100_u64);
        }
        
        // 2. Read analog temperature from T1
        let temp_analog_raw = read_adc(ADC1);
        temp_analog = ((temp_analog_raw as u32 * 550) / 1023 + 50) as i16;
        
        // 3. Read pH value from Po
        let ph_raw = read_adc(ADC0);
        
        // Calculate pH using the calibrated formula - INVERTED relationship
        let ph_value = if ph_raw >= PH_MIN_ADC {
            // Above maximum ADC value, use minimum pH
            PH_MIN
        } else if ph_raw <= PH_MAX_ADC {
            // Below minimum ADC value, use maximum pH
            PH_MAX
        } else {
            // Linear interpolation between min and max
            let adc_range = PH_MIN_ADC - PH_MAX_ADC;
            let ph_range = PH_MAX - PH_MIN;
            let adc_position = PH_MIN_ADC - ph_raw;
            
            PH_MIN + ((adc_position as u32 * ph_range as u32) / adc_range as u32) as u16
        };
        
        // Finish LED blink
        port::B5::set_low();
        
        // Send values
        if CALIBRATION_MODE {
            // Calibration output format
            uart_send_string("CALIBRATION MODE (INVERTED):\r\n");
            uart_send_string("=================\r\n");
            
            uart_send_string("pH RAW ADC: ");
            uart_send_integer(ph_raw, 10);
            uart_send_string("\r\n");
            
            uart_send_string("pH VALUE: ");
            uart_send_ph(ph_value);
            uart_send_string("\r\n");
            
            uart_send_string("pH VALUE CALCULATION (Inverted):\r\n");
            uart_send_string("ADC Range: ");
            uart_send_integer(PH_MIN_ADC, 10);
            uart_send_string(" (pH 2.0) - ");
            uart_send_integer(PH_MAX_ADC, 10);
            uart_send_string(" (pH 14.0)\r\n");
            
            uart_send_string("pH Range: ");
            uart_send_ph(PH_MIN);
            uart_send_string(" - ");
            uart_send_ph(PH_MAX);
            uart_send_string("\r\n");
            
            uart_send_string("Temperature: ");
            if ds18b20_found {
                uart_send_temperature(temp_value);
            } else {
                uart_send_string("Sensor not detected");
            }
            uart_send_string("\r\n");
            
            uart_send_string("Calibration Guide:\r\n");
            uart_send_string("- Lime Juice ~pH 2.2 (acidic) - should read HIGH ADC\r\n");
            uart_send_string("- Water ~pH 7.0 (neutral) - should read MEDIUM ADC\r\n");
            uart_send_string("- Baking Soda ~pH 8.4 (alkaline) - should read LOW ADC\r\n");
            uart_send_string("=================\r\n");
        } else {
            // Normal output format
            uart_send_string("Measurements:\r\n");
            
            // Digital temperature (T2)
            uart_send_string("Temperature (Digital T2): ");
            if ds18b20_found {
                uart_send_temperature(temp_value);
                uart_send_string(" (DS18B20 Found)");
            } else {
                uart_send_string("Sensor not detected");
            }
            uart_send_string("\r\n");
            
            // Analog temperature (T1)
            uart_send_string("Temperature (Analog T1): ");
            uart_send_temperature(temp_analog);
            uart_send_string(" (ADC: ");
            uart_send_integer(temp_analog_raw, 10);
            uart_send_string(")");
            if temp_analog_raw < 100 || temp_analog_raw > 900 {
                uart_send_string(" - Likely disconnected/invalid");
            }
            uart_send_string("\r\n");
            
            // pH reading
            uart_send_string("pH: ");
            uart_send_ph(ph_value);
            uart_send_string(" (ADC: ");
            uart_send_integer(ph_raw, 10);
            uart_send_string(")\r\n");
            
            // Raw digital temperature values
            if ds18b20_found {
                uart_send_string("DS18B20 Raw: 0x");
                uart_send_integer(temp_raw_low as u16, 16);
                uart_send_string(" 0x");
                uart_send_integer(temp_raw_high as u16, 16);
                uart_send_string("\r\n");
            }
        }
        
        // Wait before next reading
        ruduino::delay::delay_ms(READING_INTERVAL_MS.into());
    }
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

// Send a temperature value over UART (format: XX.X)
fn uart_send_temperature(value: i16) {
    // Handle negative temperatures
    if value < 0 {
        uart_send_byte(b'-');
        uart_send_decimal(-value as u16, 1);
    } else {
        uart_send_decimal(value as u16, 1);
    }
}

// DS18B20 1-Wire Protocol Functions

// Reset the 1-Wire bus and check for device presence
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
