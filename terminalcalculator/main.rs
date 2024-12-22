use std::io::{self, Write};

fn calc() { 
    // Prompt the user for input
    print!("Please enter your first number: ");

    // Make sure the prompt is printed to the screen immediately
    io::stdout().flush().unwrap();

    // Declare a mutable variable to hold the user input
    let mut n1 = String::new();

    // Read the user input into the 'name' variable
    io::stdin().read_line(&mut n1).unwrap();

    // Trim the input to remove the newline character at the end
    let n1 = n1.trim();

    // Prompt the user for input
    print!("Please enter your second number: ");

    // Make sure the prompt is printed to the screen immediately
    io::stdout().flush().unwrap();

    // Declare a mutable variable to hold the user input
    let mut n2 = String::new();

    // Read the user input into the 'name' variable
    io::stdin().read_line(&mut n2).unwrap();

    // Trim the input to remove the newline character at the end
    let n2 = n2.trim();

    let in1: f64 = n1.parse().unwrap();
    let in2: f64 = n2.parse().unwrap();
    let add = in1 + in2;
    let sub = in1 - in2;
    let mul = in1 * in2;
    let div = in1 / in2;
    println!("The addition is {}", add);
    println!("The subtraction is {}", sub);
    println!("The multiplication is {}", mul);
    println!("The division is {}", div);
}

fn main(){
    calc();
}