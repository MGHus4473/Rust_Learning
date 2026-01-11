use rand::Rng;
use std::io;

fn main() {
    println!("Guseeing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number is {secret_number}");
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
