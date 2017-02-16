// calls for an external library
extern crate rand;

// calls library functions to support program
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//spawns function called main
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 50);
//defines program loop
    loop {
        println!("Please input your guess.");
// 'mut' establishes mutable variable
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");
// 'u32' details integer type to be placed in the string and
// establishes panic method in the function above
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
 // establishes value size conditionals

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
// ends loop once satisfactory condition is met through
//the match statement
                break;
            }
        }
    }
}
