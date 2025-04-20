use ruduino::Register;

// UART configuration
pub const BAUD_RATE: u32 = 9600;
pub const CPU_FREQUENCY: u32 = 16_000_000;
pub const UBRR_VALUE: u16 = (CPU_FREQUENCY / (16 * BAUD_RATE) - 1) as u16;

// UART Register definitions
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

// Initialize the UART
pub fn initialize() {
    // Set baud rate
    let ubrr = UBRR_VALUE;
    UBRR0H::write((ubrr >> 8) as u8);
    UBRR0L::write(ubrr as u8);
    
    // Enable transmitter
    UCSR0B::write(TXEN0);
    
    // Set frame format: 8 data bits, 1 stop bit, no parity
    UCSR0C::write(UCSZ01 | UCSZ00);
}

// Send a single byte over UART
pub fn send_byte(data: u8) {
    // Wait for the transmit buffer to be empty
    while UCSR0A::read() & UDRE0 == 0 {}
    
    // Send the data
    UDR0::write(data);
}

// Send a string over UART
pub fn send_string(s: &str) {
    for byte in s.bytes() {
        send_byte(byte);
    }
}

// Send a decimal number with specified number of decimal places
pub fn send_decimal(value: u16, decimal_places: u8) {
    let mut divisor = 1;
    for _ in 0..decimal_places {
        divisor *= 10;
    }
    
    let whole = value / divisor;
    let fraction = value % divisor;
    
    // Send whole part
    send_integer(whole, 10);
    
    // Send decimal point
    send_byte(b'.');
    
    // Send fractional part with leading zeros if needed
    let mut fraction_divisor = divisor / 10;
    while fraction_divisor > 0 {
        let digit = (fraction / fraction_divisor) % 10;
        send_byte(b'0' + digit as u8);
        fraction_divisor /= 10;
    }
}

// Send an integer value with the specified base
pub fn send_integer(mut value: u16, base: u16) {
    const BUFFER_SIZE: usize = 16;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut i = BUFFER_SIZE;
    
    // Handle the case of zero separately
    if value == 0 {
        send_byte(b'0');
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
        send_byte(buffer[j]);
    }
}
