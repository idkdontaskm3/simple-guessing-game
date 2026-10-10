//use std::io::{};
use std::cmp::Ordering;
use eframe::egui;
use std::process;

fn main() {
    env_logger::init();
    let native_options = eframe::NativeOptions::default();
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
        Self { secret, guess, guess_input, correct, total_guesses, ..Self::default() }
    }
}

impl eframe::App for SimpleGuessingGame {
    fn ui (&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("SimpleGuessingGame v0.2.0-alpha");
            ui.horizontal(|ui| {
                ui.label("Input Your Guess: ");
                ui.text_edit_singleline(&mut self.guess_input);
                if ui.button("Guess").clicked() {
                    self.guess = self.guess_input.trim().parse().ok();
                    self.total_guesses = self.total_guesses + 1;
                }
            });

            ui.horizontal( |ui| {
                ui.label(format!("Your guess: {:?}", self.guess.unwrap_or(0)));
                ui.label(format!("Correct: {}", self.correct));
                ui.label(format!("Total Guesses: {:?}", self.total_guesses));
            });

            ui.horizontal(|ui| {
                if ui.button("Exit").clicked() {
                    process::exit(0);
                }
            });
        });

        self.correct = match self.guess.unwrap_or(0).cmp(&self.secret) {
            Ordering::Greater => String::from("nah, too high"),
            Ordering::Less => String::from("nah, too low"),
            Ordering::Equal => String::from("Correct!"),
        };
    }
}