use ruduino::Pin;
use ruduino::cores::current::port::D5;

pub fn initialize() {
    D5::set_output();
}

pub fn activate_bubbles() {
    D5::set_high();
}

pub fn deactivate_bubbles() {
    D5::set_low();
}
