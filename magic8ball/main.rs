use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Please enter a Yes or No Question: ");

    // Make sure the prompt is printed to the screen immediately
    io::stdout().flush().unwrap();

    // Declare a mutable variable to hold the user input
    let mut n1 = String::new();

    // Read the user input into the 'name' variable
    io::stdin().read_line(&mut n1).unwrap();

    // Trim the input to remove the newline character at the end
    let _n1 = n1.trim();

    let num1 = vec![2, 3];
    let address1 = &num1 as *const Vec<i32>;
    let number1 = address1 as i32;

    if number1 % 13 == 0 {
        println!("
Yes, Certainly, Absolutely
");
    } else if number1 % 13 == 1 || number1 % 13 == -1 {
        println!("
No, I don't think so
");
    } else if number1 % 13 == 2 || number1 % 13 == -2 {
        println!("
Hell Yeah
");
    } else if number1 % 13 == 3 || number1 % 13 == -3 {
        println!("
Oh, Hell No
");
    } else if number1 % 13 == 4 || number1 % 13 == -4 {
        println!("
My Gut Tells Me Yes
");
    } else if number1 % 13 == 5 || number1 % 13 == -5 {
        println!("
Did You Wash Yo Ass Today? So That's A No
");
    } else if number1 % 13 == 6 || number1 % 13 == -6 {
        println!("
Seems like a Yes
");
    } else if number1 % 13 == 7 || number1 % 13 == -7 {
        println!("
Obviously No
");
    } else if number1 % 13 == 8 || number1 % 13 == -8 {
        println!("
100% Yes
");
    } else if number1 % 13 >= 9 || number1 % 13 <= -9 {
        println!("
No... Just No
");
    }
}