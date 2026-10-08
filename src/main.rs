use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    let version = "0.1.0n"; // n- nightly a- alpha b- beta p- production
    let secret = rand::random_range(0..=100);

    println!("######################");
    println!("# simpleGuessingGame #");
    println!("######################\n");

    println!("v{}\n", {version});

    print!("Input Your Guess [0-100]: ");
    io::stdout().flush().expect("Couldn't flush stdout");

    let guess: u8 = fetch_guess();
    println!("your guess: {}", guess);
    
    println!("DEBUG-SECRET: {}", {secret});
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