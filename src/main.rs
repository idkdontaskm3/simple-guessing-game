//use std::io::{};
//use std::cmp::Ordering;
use eframe::egui;
fn main() {
    env_logger::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("SimpleGuessingGame", native_options,Box::new(|cc| Ok(Box::new(SimpleGuessingGame::new(cc))))).unwrap();
}

#[derive(Default)]
struct SimpleGuessingGame {
    guess_input: String,
    guess: Option <u8>,
    secret: u8,
}
impl SimpleGuessingGame {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SimpleGuessingGame {
    fn ui (&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let secret = rand::random_range(0..100);

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("SimpleGuessingGame v0.2.0n");
            ui.horizontal(|ui| {
                ui.label("Input Your Guess: ");
                ui.text_edit_singleline(&mut self.guess_input);
                if ui.button("Guess").clicked() {
                    self.guess = self.guess_input.trim().parse().ok();
                }
            });

            ui.horizontal( |ui| {
                ui.label(format!("Your guess: {:?}", self.guess.unwrap_or(0)));
                ui.label(format!("DEBUG_SECRET: {:?}", self.secret));
            });
        });
    }
}