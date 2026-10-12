//use std::io::{};
use std::cmp::Ordering;
use eframe::egui;
use std::process;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 115.0]),
        ..Default::default()
    };
    eframe::run_native("SimpleGuessingGame", native_options,Box::new(|cc| Ok(Box::new(SimpleGuessingGame::new(cc))))).unwrap();
}

#[derive(Default)]
struct SimpleGuessingGame {
    secret: u8,
    guess_input: String,
    guess: Option <u8>,
    correct: String,
    total_guesses: u16,
}

impl SimpleGuessingGame {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let secret: u8 = rand::random_range(0..100);
        let guess_input: String = String::from("");
        let guess: Option <u8> = None;
        let correct: String = String::from("False");
        let total_guesses: u16 = 0;
        Self { secret, guess_input, guess, correct, total_guesses, ..Self::default() }
    }
}

impl eframe::App for SimpleGuessingGame {
    fn ui (&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("SimpleGuessingGame v0.2.0-beta");
            if self.guess != Some(self.secret) {
                ui.horizontal(|ui| {
                    ui.label("Input Your Guess [0-100]: ");
                    let response = ui.text_edit_singleline(&mut self.guess_input);
                    let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if ui.button("Guess").clicked() || enter_pressed {
                        self.guess = self.guess_input.trim().parse().ok();
                        self.total_guesses = self.total_guesses + 1;
                        response.request_focus();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Your guess: {:?}", self.guess.unwrap_or(0)));
                    ui.label(format!("Correct: {}", self.correct));
                    ui.label(format!("Total Guesses: {:?}", self.total_guesses));
                });

                ui.horizontal(|ui| {
                    if ui.button("Exit").clicked() {
                        process::exit(0);
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(format!("Correct! Total Guesses: {:?}", self.total_guesses));
                });

                ui.horizontal_top(|ui| {
                    if ui.button("Play Again?").clicked() {
                        self.secret = rand::random_range(0..100);
                        self.guess_input = String::from("");
                        self.guess = None;
                        self.correct = String::from("False");
                        self.total_guesses = 0;
                    }
                    
                    if ui.button("Exit").clicked() {
                        process::exit(0);
                    }
                });
            }
        });

        self.correct = match self.guess.unwrap_or(0).cmp(&self.secret) {
            Ordering::Greater => String::from("nah, too high"),
            Ordering::Less => String::from("nah, too low"),
            Ordering::Equal => String::from("Correct!"),
        };
    }
}