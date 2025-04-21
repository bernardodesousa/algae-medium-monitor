#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

use ruduino::delay;

mod display;
mod ph;
mod adc;
mod temperature;
mod sensor_manager;
mod display_controller;
mod air;

use sensor_manager::SensorManager;
use display_controller::DisplayController;

// Constants for timing
const DISPLAY_REFRESH_DELAY_MS: u64 = 2; // Delay between display refreshes (ms)
const DISPLAY_TIME_PER_READING: u64 = 3000; // Display each reading for 3 seconds

#[no_mangle]
pub extern "C" fn main() {
    // Create and initialize controllers
    let mut sensor_manager = SensorManager::new();
    let mut display_controller = DisplayController::new(DISPLAY_TIME_PER_READING);
    
    // Initialize hardware
    sensor_manager.initialize();
    display_controller.initialize();
    air::initialize(); // Initialize the air module
    
    // Start reading temperature for initial display
    sensor_manager.start_initial_temperature_reading();
    
    // Time tracking
    let mut current_time: u64 = 0;

    
    // Main loop
    loop {
        // Update display with current sensor values
        display_controller.update_display(&sensor_manager.values);
        
        // Update sensors - passing whether we're showing temperature
        sensor_manager.update(
            current_time, 
            display_controller.is_showing_temperature()
        );
        
        // Check if it's time to switch display modes
        display_controller.check_mode_switch(current_time);
        
        // Short delay for display timing
        delay::delay_ms(DISPLAY_REFRESH_DELAY_MS);
        
        // Update time counter
        current_time += DISPLAY_REFRESH_DELAY_MS;

        // Activate bubbles for 30 seconds every 10 minutes (600 seconds)
        if (current_time % 600000) < 30000 {
            air::activate_bubbles();
        } else {
            air::deactivate_bubbles();
        }
    }
}
