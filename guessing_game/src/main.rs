use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a seeded random integer between 1 and 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Game loop
    loop {
        println!("Please input your guess.");

        // Declare a mutable empty string
        let mut guess = String::new();

        // Read input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Ignoring a non-number guess and prompting again instead of crashing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Evaluate the guess against the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Exit on success
                break;
            },
        }
    }
}
