use std::io;
use colored::*;
use rand::Rng;


fn main() {
    let text = "Welcome to the Rust Password Generator!";

    println!("{}", text.red());


    println!("{}","Enter the length of the password:".blue());
    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line!");

    let length:usize = length.trim().parse().expect("Invalid. Please enter  a number");


    let password = generate_password(length);

    println!("Your Password: {}", password.green());
   
}

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric )as char)
        .collect();

    password


}
