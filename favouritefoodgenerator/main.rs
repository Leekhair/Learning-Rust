use std::io::{self, Write};

fn main() {
    let _ff = "I don't know their favourite food";

    // Prompt the user for input
    print!("Please enter someones name: ");
    // Make sure the prompt is printed to the screen immediately
    io::stdout().flush().unwrap();

    // Declare a mutable variable to hold the user input
    let mut name = String::new();

    // Read the user input into the 'name' variable
    io::stdin().read_line(&mut name).unwrap();

    // Trim the input to remove the newline character at the end
    let name = name.trim();

    match name {
        "Alexander" => println!("Aron's favourite food is Sushi"),
        "William" => println!("Liam's favourite food is Pizza"),
        "Mateo" => println!("Matteo's favourite food is Pizza"),
        "John" => println!("Jonathan's favourite food is Nocciola Flavoured Greek Yogurt"),
        _ => println!("I don't know their favourite food"),  // Default case
    }
}
