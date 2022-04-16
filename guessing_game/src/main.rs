use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let mut guess = String::new();    // mut creates a mutable variable bound to a new(empty) string
                                      // without the mut keyword, the variable is immutable


    let secret_number = thread_rng().gen_range(1..101).to_string();

    io::stdin().read_line(&mut guess)    //taking input and storing it to guess
        .expect("Failed to read line");
        /*The full job of read_line is to take whatever the user types into standard input
         and append that into a string [without overwriting its contents], 
         so we therefore pass that string as an argument. 
         The string argument needs to be mutable so the method can change the string’s content.*/
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    if 5>3 {
        println!("5 is greater than 3");
    }
    eprintln!("The secret number is: {}", secret_number);
    
}
