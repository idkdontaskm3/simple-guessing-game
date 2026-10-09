use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    let version = "0.1.0a"; // n- nightly a- alpha b- beta p- production
    let secret = rand::random_range(0..=100);

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
            Ordering::Equal => { println!("correct! total guesses: {}", total_guesses); break; },
        }
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