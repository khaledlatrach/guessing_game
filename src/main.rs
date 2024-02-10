use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

//crate...crate is a collection of Rust source code files
// prelude...
//parentheses
//brackets
//Bind a variable
// An associated function is a function that’s implemented on a type
//The :: syntax in the ::new line indicates that new is an associated function of the String type
//The purpose of these Result types is to encode error-handling information.
//enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.

//cargo build ===> cargo.lock created for the first time
// Then run cargo update to change the crate vesrion..
//trait
//insatiable لا يشبع
