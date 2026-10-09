use std::io::{self};
//use std::cmp::Ordering;
use eframe::egui;
fn main() {
    env_logger::init();

    //let secret = rand::random_range(0..=100);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("SimpleGuessingGame", native_options,Box::new(|cc| Ok(Box::new(SimpleGuessingGame::new(cc)))));

    /*
    println!("######################");
    println!("# simpleGuessingGame #");
    println!("######################\n");

    println!("v{}\n", {version});

    println!("DEBUG-SECRET: {}", {secret});

    print!("Input Your Guess [0-100]: ");
    io::stdout().flush().expect("Couldn't flush stdout, aka ur code is fried or sum");

    let mut total_guesses: u16 = 0;

    loop {
        let guess: u8 = fetch_guess();

        total_guesses = total_guesses + 1;
        
        match guess.cmp(&secret) {
            Ordering::Less => {
                print!("too low! total guesses: {}\ninput next: ", total_guesses);
                io::stdout().flush().expect("Couldn't flush stdout, line 30");
            },
            Ordering::Greater => {
                print!("too high! total guesses: {}\ninput next: ", total_guesses);
                io::stdout().flush().expect("Couldn't flush stdout, line 35");
            },
            Ordering::Equal => {
                println!("correct! total guesses: {}", total_guesses);
                break Ok(());
            },
        }
    }
    */
}

#[derive(Default)]
struct SimpleGuessingGame {
    guess_input: String,
    guess: Option <u8>,
}
impl SimpleGuessingGame {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SimpleGuessingGame {
    fn ui (&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("SimpleGuessingGame v0.2.0n");
            ui.horizontal(|ui| {
                ui.label("Input Your Guess: ");
                ui.text_edit_singleline(&mut self.guess_input);
                if ui.button("Guess").clicked() {
                    self.guess = self.guess_input.trim().parse().ok();
                }
            });
        });
    }
}
/*
fn fetch_guess() -> u8 {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read!");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid, try again"),
        }
    }
}
*/