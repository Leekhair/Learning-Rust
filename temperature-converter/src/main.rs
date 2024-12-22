use std::io::{self, Write};

fn calc() { 
    // Prompt the user for input
    print!("Please enter a tempurature: ");

    // Make sure the prompt is printed to the screen immediately
    io::stdout().flush().unwrap();

    // Declare a mutable variable to hold the user input
    let mut n1 = String::new();

    // Read the user input into the 'name' variable
    io::stdin().read_line(&mut n1).unwrap();

    // Trim the input to remove the newline character at the end
    let n1 = n1.trim();

    // Prompt the user for input
    print!("Celsius to Farenheit or Farenheit to Celsius ( 0 / 1 ): ");

    // Make sure the prompt is printed to the screen immediately
    io::stdout().flush().unwrap();

    // Declare a mutable variable to hold the user input
    let mut n2 = String::new();

    // Read the user input into the 'name' variable
    io::stdin().read_line(&mut n2).unwrap();

    // Trim the input to remove the newline character at the end
    let n2 = n2.trim();

    let in1: i32 = n1.parse().unwrap();
    let in2: i32 = n2.parse().unwrap();
    let mut converted: i32 = 43;

    if in2 == 0{
        converted = (in1 * 9/5) + 32;
    } else if in2 == 1{
        converted = (in1 - 32) * 5/9;
    }

    println!("The converted tempurature is {}", converted);

}

fn main(){
    calc();
}