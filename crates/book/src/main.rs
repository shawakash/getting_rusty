use rand::Rng;
use std::cmp::Ordering;
use std::{io, process};
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(-100..=100);
    loop {
        println!("\nTake a guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("You must have entered a string");
        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bingo!");
                process::exit(0);
            }
        }
        println!("\nWant to continue: (y/n)");
        let mut do_continue = String::new();
        io::stdin()
            .read_line(&mut do_continue)
            .expect("You must have entered a string");
        if do_continue.trim() != "y" || do_continue.trim() != "Y" {
            break;
        }
    }
}

