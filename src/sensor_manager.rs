use crate::temperature;
use crate::adc;
use crate::ph;

// Sensor state enum to track sensor operations
enum SensorState {
    Idle,
    TemperatureConverting(u64),  // Start time of conversion
    TemperatureReady,
    PHReading,
    PHReady
}

// Current sensor values 
pub struct SensorValues {
    pub temperature: f32,
    pub ph: f32,
}

// The Sensor Manager handles all sensor-related operations
pub struct SensorManager {
    state: SensorState,
    pub values: SensorValues,
}

impl SensorManager {
    // Create a new sensor manager with default values
    pub fn new() -> Self {
        // Initialize with default values
        let values = SensorValues {
            temperature: 25.0,
            ph: 7.0,
        };
        
        // Start in idle state
        SensorManager {
            state: SensorState::Idle,
            values,
        }
    }
    
    // Initialize the sensors
    pub fn initialize(&self) {
        adc::initialize();
        temperature::initialize();
    }
    
    // Start the initial temperature reading
    pub fn start_initial_temperature_reading(&mut self) {
        temperature::start_temperature_conversion();
        self.state = SensorState::TemperatureConverting(0);
    }
    
    // Update sensor operations based on current state and display mode
    pub fn update(&mut self, current_time: u64, is_showing_temperature: bool) {
        match self.state {
            SensorState::Idle => {
                // Idle state - no ongoing sensor operations
                // Check if we need to start a new sensor reading based on display mode
                if is_showing_temperature {
                    // Start reading pH in the background for next switch
                    self.state = SensorState::PHReading;
                } else {
                    // Start temperature conversion for next switch
                    temperature::start_temperature_conversion();
                    self.state = SensorState::TemperatureConverting(current_time);
                }
            },
            
            SensorState::TemperatureConverting(start_time) => {
                // Check if conversion time has elapsed
                if current_time >= start_time + temperature::TEMP_CONVERSION_TIME_MS as u64 {
                    // Conversion should be complete, move to ready state
                    self.state = SensorState::TemperatureReady;
                }
                // Otherwise keep waiting
            },
            
            SensorState::TemperatureReady => {
                // Read the temperature value
                if let Some(temp) = temperature::read_temperature_after_conversion() {
                    self.values.temperature = temp as f32 / 10.0;
                }
                
                // Move back to idle state
                self.state = SensorState::Idle;
            },
            
            SensorState::PHReading => {
                // Read pH (this is fast, so we do it immediately)
                let ph_raw = adc::read(adc::ADC0);
                let ph_raw_value = ph::adc_to_ph(ph_raw);
                self.values.ph = ph_raw_value as f32 / 100.0;
                
                // Mark pH as ready
                self.state = SensorState::PHReady;
            },
            
            SensorState::PHReady => {
                // pH reading is complete, return to idle
                self.state = SensorState::Idle;
            }
        }
    }
} 