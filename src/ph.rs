// pH sensor module

// Calibration mode - set to true when calibrating
// pub const CALIBRATION_MODE: bool = true;

// Known pH values for calibration
// pub const PH_ACID: f32 = 2.2;     // Lime juice approximate pH
// pub const PH_NEUTRAL: f32 = 7.0;  // Water approximate pH
// pub const PH_ALKALINE: f32 = 8.4; // Baking soda solution approximate pH

// pH conversion parameters
// These parameters map the ADC values to pH values
// Based on observations: Higher ADC = LOWER pH, Lower ADC = HIGHER pH
pub const PH_MIN_ADC: u16 = 1020;  // ADC value corresponding to pH MIN
pub const PH_MAX_ADC: u16 = 650;   // ADC value corresponding to pH MAX
pub const PH_MIN: u16 = 200;       // pH 2.00 * 100
pub const PH_MAX: u16 = 1400;      // pH 14.00 * 100

// Convert raw ADC value to pH
pub fn adc_to_ph(ph_raw: u16) -> u16 {
    // Calculate pH using the calibrated formula - INVERTED relationship
    if ph_raw >= PH_MIN_ADC {
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
    }
}
