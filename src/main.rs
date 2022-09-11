use std::io;

fn main() {
    println!("!!!Welcome to the guessing game!!!");
    println!("Please enter a Number guess!!");
    let mut guess = String::new();
    io::stdin()
        .read_line(& mut guess)
        .expect("Failed to read the line input!!");
    println!("Your Guess is --> {guess}");
    
}
