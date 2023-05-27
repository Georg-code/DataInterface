#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use serialport::available_ports;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_simple_native("BoatData", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ports = serial_ports();

            ui.heading("Maturaboot Interface");
            ui.horizontal(|ui| {
                ui.label("Serial Ports:");
                ui.separator();
                ui.label(format!("{}", ports.len()));
            });
            if ports.is_empty() {
                ui.label("No serial ports found.");
            } else {
                for port in &ports {
                    if ui.add(egui::Button::new(format!("{}", &port.port_name))).clicked() {
                    println!("Clicked");
                    }
                }
            }

        });
    })
}

fn serial_ports() -> Vec<serialport::SerialPortInfo>{
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


