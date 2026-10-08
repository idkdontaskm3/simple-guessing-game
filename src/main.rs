use std::io::{self};
use std::cmp::Ordering;

fn main() {
    let version = "0.1.0n"; // n- nightly a- alpha b- beta p- production

    println!("######################");
    println!("# simpleGuessingGame #");
    println!("######################\n");

    println!("v{}\n", {version});

    let secret = rand::random_range(0..=100);
    //let mut guess = String::new();

    //println!("Input a Guess [0-100]:");
    //io::stdin().read_line(&mut guess).expect("Failed to read");
    //println!("Your guess: {}", guess.trim());
    
    println!("DEBUG-SECRET: {}", {secret});
}

fn fetch_guess() -> u8 {
    loop {
        let mut guess = String::new();
        println!("Input your guess [0-100]:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read!");

        match guess.trim() {
            Ok(num) => return num,
            Err(_) => println!("Invalid, try again"),
        }
    }
}