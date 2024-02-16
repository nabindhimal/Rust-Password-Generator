use std::io;
use colored::*;
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let text = "Welcome to the Rust Password Generator!";
    println!("{}", text.red());

    println!("{}","Enter the length of the password:".blue());
    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line!");

    let length: usize = length.trim().parse().expect("Invalid. Please enter a number");

    let password = generate_password(length);

    println!("Your Password: {}", password.green());
}

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let alphanumeric_length = length * 3 / 4;
    let additional_length = length - alphanumeric_length;

    let alphanumeric_password: String = (0..alphanumeric_length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();

    let additional_characters: &str = "!@#$%^&*";
    let additional_numbers: &str = "0123456789";

    let additional_chars: String = (0..additional_length / 2)
        .map(|_| {
            let index = rng.gen_range(0..additional_characters.len());
            additional_characters.chars().nth(index).unwrap()
        })
        .collect();

    let additional_numbers: String = (0..additional_length / 2)
        .map(|_| {
            let index = rng.gen_range(0..additional_numbers.len());
            additional_numbers.chars().nth(index).unwrap()
        })
        .collect();

    let combined_chars = format!("{}{}{}", alphanumeric_password, additional_chars, additional_numbers);

    let mut password_chars: Vec<char> = combined_chars.chars().collect();
    password_chars.shuffle(&mut rng);

    password_chars.into_iter().collect()
}

