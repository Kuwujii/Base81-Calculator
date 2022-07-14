use eframe::egui; //Egui wrapper

mod constants;

fn main() {
    let options: eframe::NativeOptions = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(50.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(constants::APP_NAME, options, Box::new(|_cc| Box::new(B81C::default())));
}

struct B81C { //Struckt with all the information required to work
    current_num: constants::Num,
    operation: String,
    memory: constants::Num
}

impl Default for B81C {
    fn default() -> Self {
        Self {
            current_num: constants::Num::default(),
            operation: "".to_string(),
            memory: constants::Num::default()
        }
    }
}

impl eframe::App for B81C { //The actual window
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

    }
}
