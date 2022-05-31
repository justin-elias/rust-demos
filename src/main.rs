use std::io;
use std::ops::Range;
use rand::Rng;

fn main() {

    loop {
        println!("Welcome! Choose the game you want to play: " );
        println!("1. Number Guess");
        println!(" Enter 'exit' to quit");
        println!("Make choice: ");

        // Take user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<String>().unwrap();

        // take action on user input
        // have to cast input from String to &str for the matching
        match input.as_str() {
            "1" => number_guess(),
            "exit" => break,
            _ => println!("Invalid input"),
        }
    }
}

fn number_guess() {
    const RANGE: Range<i32> = 0..11;

    // Generate a random number
    let number_to_guess = rand::thread_rng().gen_range(RANGE);

    // Get the user's input with a mutable variable
    println!("Guess a number between {} and {}", RANGE.start, RANGE.end);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    // Check if the user's input is correct
    if input == number_to_guess {
        println!("You win!");
    } else {
        println!("Better Luck Next Time! The number was {}", number_to_guess);
        println!()
    }
}
