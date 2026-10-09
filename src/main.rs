use std::io::{self, Write};
use std::cmp::Ordering;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let version = "0.2.0n"; // n- nightly a- alpha b- beta p- production
    let secret = rand::random_range(0..=100);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("SimpleGuessingGame", native_options,Box::new(|cc| Ok(Box::new(SimpleGuessingGame::new(cc)))));

    println!("######################");
    println!("# simpleGuessingGame #");
    println!("######################\n");

    println!("v{}\n", {version});

    //println!("DEBUG-SECRET: {}", {secret});

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
}

#[derive(Default)]
struct SimpleGuessingGame {}
impl SimpleGuessingGame {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for SimpleGuessingGame {
    fn ui (&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // TODO - like everything ;-;
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("SimpleGuessingGame");
        });
    }
}

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