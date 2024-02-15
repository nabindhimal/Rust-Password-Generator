use std::io;
use colored::*;
use rand::Rng;
use rand::seq::SliceRandom;  // Import the necessary trait

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
    let alphanumeric_password: String = (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();

    let additional_characters: &str = "!@#$%^&*";
    let additional_numbers: &str = "0123456789";
    let additional_length = length / 4; // You can adjust this ratio as needed

    let additional_chars: Vec<char> = (0..additional_length)
        .map(|_| {
            let index = rng.gen_range(0..additional_characters.len());
            additional_characters.chars().nth(index).unwrap()
        })
        .collect();

    let mut additional_numbers: Vec<char> = (0..additional_length)
        .map(|_| {
            let index = rng.gen_range(0..additional_numbers.len());
            additional_numbers.chars().nth(index).unwrap()
        })
        .collect();

    let mut combined_chars = additional_chars;
    combined_chars.append(&mut additional_numbers);

    combined_chars.shuffle(&mut rng);

    let final_password: String = alphanumeric_password
        .chars()
        .zip(combined_chars.iter())
        .flat_map(|(a, b)| vec![a, *b])
        .collect();

    final_password
}


