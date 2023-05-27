use crate::serial_communication;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BoatInterface {
    // Example stuff:
    label: String,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    organs: IterHeart,
}

impl Default for BoatInterface {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Boat App".to_owned(),
            value: 2.7,
            organs: IterHeart::new(vec![
                "❤️", "🧡", "💛", "💚", "💙", "💜", "🖤", "🤎", "🤍",
                "💔", "hash col stinks", "💓", "💕", "💖", "💘", "💝", "💞", "💟",
                "🫀", "🫁", "🧠", "🫂", "🤲"
            ]),
    
            
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct IterHeart {
    i: usize,
    organs: Vec<String>,
}

impl IterHeart {
    fn new(organs: Vec<&str>) -> Self {

        Self {
            i: 0,
            organs: organs.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn next(&mut self) -> &str {
        self.i += 1;
        self.organs[self.i % self.organs.len()]
        .as_str()
    }
 }

impl BoatInterface {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BoatInterface {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let ports = serial_communication::serial_ports();

      
        #[cfg(not(target_arch = "wasm32"))] 
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {


            ui.heading("Select Serial Port");

            if ports.len() == 0 {
                ui.label("No serial ports found. Searching...");
            } else {
                for port in &ports {
                    ui.horizontal(|ui| {
                    if ui.add(egui::Button::new(format!("{}", &port.port_name))).clicked() {   
                    println!("Clicked");
                    }
                });
            }
          
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;  
                    ui.label(format!("Made with {} by Georg", self.organs.next()));
                   
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("No Serial Port Selected");
            ui.label("Select a serial port from the side panel to begin.");

        });

    }
}
