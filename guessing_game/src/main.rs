// TAKING THE USER INPUT
use rand::{Rng, rng};
use std::cmp::Ordering;
use std::io;
// io library is used to perform user input operations
//  it is present in the standard std library

fn main() {
    println!("Guess the number!");

    // GENERATING A RANDOM NUMBER (GUESSING A NUMBER)

    // rust has no standard library to support generating random numbers

    // using a crate dependency in our program
    let secret_number: u32 = rand::rng().random_range(1..=100);
    // using SHADOWING to create variable with the same name
    loop {
        println!("Please input your guess (1-100):");
        let mut guess = String::new();
        io::stdin()
            // imported from io library to handle user input
            .read_line(&mut guess)
            //  read_line is to take whatever the user types into standard input
            // and append that into a string
            // & is used to create a mutable reference to guess
            //  as reference or access to guess is immutable
            //  we use & as references are immutable in nature
            .expect("Failed to read line!");
        // if fails print the above and compiles the program

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        // trim is useed as when we provide an input and click "enter", a \n next line character also gets added to the input.
        // Hence we trim it out before parsing into some integer

        // Comparing both the values
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
