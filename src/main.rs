use serialport::SerialPort;
use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/tty", 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    // Initialize printer
    initialize_printer(&mut *port);

    // Set line spacing
    set_line_spacing(&mut *port, 24); // Example: Set line spacing to 24/180 inch

    // Print text
    print_text(&mut *port, "Hello, World!\n");
}

fn initialize_printer(port: &mut dyn SerialPort) {
    send_command(port, b"\x1B\x40"); // ESC @
}

fn set_line_spacing(port: &mut dyn SerialPort, spacing: u8) {
    send_command(port, &[0x1B, b'3', spacing]); // ESC 3 <n>
}

fn print_text(port: &mut dyn SerialPort, text: &str) {
    send_command(port, text.as_bytes());
    // Send additional commands as needed, e.g., line feed
    send_command(port, b"\n"); // Line Feed
}

fn send_command(port: &mut dyn SerialPort, command: &[u8]) {
    port.write_all(command).expect("Failed to write to port");
}
