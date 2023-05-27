use serialport::available_ports;


pub fn serial_ports() -> Vec<serialport::SerialPortInfo>{
    match available_ports() {
        Ok(ports) => {
            return ports;
        }
        Err(e) => {
            eprintln!("Error listing serial ports: {}", e);
            return Vec::new();
        }
     
    }
}
