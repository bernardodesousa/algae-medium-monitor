#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::port;

// Import our modules
mod adc;
mod uart;
mod ds18b20;
mod ph;
mod display;

// Timing constants
const READING_INTERVAL_MS: u32 = 2000; // Time between readings
const DISPLAY_UPDATE_MS: u32 = 5;      // Time between display updates

#[no_mangle]
pub extern "C" fn main() {
    // Initialize ADC for pH sensor
    adc::initialize();
    
    // Initialize UART
    uart::initialize();
    
    // Initialize the status LED
    port::B5::set_output();
    
    // Initialize DS18B20 temperature sensor
    ds18b20::initialize();
    
    // Initialize the 7-segment display
    display::initialize();
    
    // Send startup message
    uart::send_string("Algae Medium Monitor Starting...\r\n");
    uart::send_string("Reading sensors\r\n");
    
    // Counter for alternating display
    let mut display_counter: u8 = 0;
    
    loop {
        // Blink LED to indicate cycle start
        port::B5::set_high();
        
        // 1. Try to read temperature from DS18B20
        let temp_value = ds18b20::read_temperature();
        
        // 2. Read pH value from ADC
        let ph_raw = adc::read(adc::ADC0);
        let ph_value = ph::adc_to_ph(ph_raw);
        
        // 3. Read analog temperature from T1 (if connected)
        let temp_analog_raw = adc::read(adc::ADC1);
        let temp_analog = ((temp_analog_raw as u32 * 550) / 1023 + 50) as i16;
        
        // Finish LED blink
        port::B5::set_low();
        
        // Update display with alternating values
        display::display_alternating(ph_value, temp_value, display_counter);
        display_counter = display_counter.wrapping_add(1);
        
        // Send values
        if ph::CALIBRATION_MODE {
            // Calibration output format
            uart::send_string("CALIBRATION MODE (INVERTED):\r\n");
            uart::send_string("=================\r\n");
            
            uart::send_string("pH RAW ADC: ");
            uart::send_integer(ph_raw, 10);
            uart::send_string("\r\n");
            
            uart::send_string("pH VALUE: ");
            uart::send_ph(ph_value);
            uart::send_string("\r\n");
            
            uart::send_string("pH VALUE CALCULATION (Inverted):\r\n");
            uart::send_string("ADC Range: ");
            uart::send_integer(ph::PH_MIN_ADC, 10);
            uart::send_string(" (pH 2.0) - ");
            uart::send_integer(ph::PH_MAX_ADC, 10);
            uart::send_string(" (pH 14.0)\r\n");
            
            uart::send_string("pH Range: ");
            uart::send_ph(ph::PH_MIN);
            uart::send_string(" - ");
            uart::send_ph(ph::PH_MAX);
            uart::send_string("\r\n");
            
            uart::send_string("Temperature: ");
            match temp_value {
                Some(temp) => uart::send_temperature(temp),
                None => uart::send_string("Sensor not detected")
            }
            uart::send_string("\r\n");
            
            uart::send_string("Calibration Guide:\r\n");
            uart::send_string("- Lime Juice ~pH 2.2 (acidic) - should read HIGH ADC\r\n");
            uart::send_string("- Water ~pH 7.0 (neutral) - should read MEDIUM ADC\r\n");
            uart::send_string("- Baking Soda ~pH 8.4 (alkaline) - should read LOW ADC\r\n");
            uart::send_string("=================\r\n");
        } else {
            // Normal output format
            uart::send_string("Measurements:\r\n");
            
            // Digital temperature
            uart::send_string("Temperature (DS18B20): ");
            match temp_value {
                Some(temp) => {
                    uart::send_temperature(temp);
                    uart::send_string(" (Sensor Found)");
                },
                None => uart::send_string("Sensor not detected")
            }
            uart::send_string("\r\n");
            
            // Analog temperature (T1)
            uart::send_string("Temperature (Analog T1): ");
            uart::send_temperature(temp_analog);
            uart::send_string(" (ADC: ");
            uart::send_integer(temp_analog_raw, 10);
            uart::send_string(")");
            if temp_analog_raw < 100 || temp_analog_raw > 900 {
                uart::send_string(" - Likely disconnected/invalid");
            }
            uart::send_string("\r\n");
            
            // pH reading
            uart::send_string("pH: ");
            uart::send_ph(ph_value);
            uart::send_string(" (ADC: ");
            uart::send_integer(ph_raw, 10);
            uart::send_string(")\r\n");
        }
        
        // Wait before next reading
        ruduino::delay::delay_ms(READING_INTERVAL_MS.into());
        
        // Update display during the waiting period
        for _ in 0..(READING_INTERVAL_MS / DISPLAY_UPDATE_MS) {
            display::update_display();
            ruduino::delay::delay_ms(DISPLAY_UPDATE_MS.into());
        }
    }
}
