use crate::display;
use crate::sensor_manager::SensorValues;

// Display mode
#[derive(PartialEq, Copy, Clone)]
pub enum DisplayMode {
    PH,
    Temperature
}

pub struct DisplayController {
    pub mode: DisplayMode,
    mode_switch_time: u64,
    display_time_per_reading_ms: u64,
}

impl DisplayController {
    // Create a new display controller
    pub fn new(display_time_per_reading_ms: u64) -> Self {
        DisplayController {
            mode: DisplayMode::Temperature, // Start with temperature
            mode_switch_time: 0,
            display_time_per_reading_ms,
        }
    }
    
    // Initialize the display
    pub fn initialize(&self) {
        display::initialize();
        display::set_auto_update(false);
    }
    
    // Update the display with current values
    pub fn update_display(&self, sensor_values: &SensorValues) {
        match self.mode {
            DisplayMode::Temperature => {
                display::display(sensor_values.temperature);
            },
            DisplayMode::PH => {
                display::display(sensor_values.ph);
            }
        }
        display::update(); // Refresh the display multiplexing
    }
    
    // Check and update display mode if needed
    pub fn check_mode_switch(&mut self, current_time: u64) -> bool {
        let mut switched = false;
        
        if current_time >= self.mode_switch_time + self.display_time_per_reading_ms {
            // Switch display mode
            self.mode = match self.mode {
                DisplayMode::Temperature => DisplayMode::PH,
                DisplayMode::PH => DisplayMode::Temperature,
            };
            
            // Reset switch timer
            self.mode_switch_time = current_time;
            switched = true;
        }
        
        switched
    }
    
    // Is the display currently showing temperature?
    pub fn is_showing_temperature(&self) -> bool {
        self.mode == DisplayMode::Temperature
    }
} 