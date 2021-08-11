// squareone: Back to Rust

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
use std::io::Write;

pub fn run() {
    let randnum = thread_rng().gen_range(1..100);

    loop {
        let mut guess = String::new();

        print!("Input a number: ");
        io::stdout().flush().expect("flush failed");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("input is not a number: {}. Retry.", guess);
                continue;
            }
        };

        match guess.cmp(&randnum) {
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
            Ordering::Greater => println!("Your guess needs to be smaller"),
            Ordering::Less => println!("Your guess needs to be larger"),
        }
    }
}
