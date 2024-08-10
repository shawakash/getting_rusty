use rand::Rng;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;
use std::{any::type_name, io, process};

pub fn cin<T: FromStr>() -> T
where
    T::Err: Debug,
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse::<T>().expect(&format!(
        "Failed to parse input, Expected Type: {}",
        type_name::<T>()
    ))
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(-100..=100);
    loop {
        println!("\nTake a guess!");

        let guess = cin::<i32>();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bingo!");
                process::exit(0);
            }
        }

        println!("\nWant to continue: (y/n)");
        let do_continue = cin::<String>();
        if do_continue.trim().to_lowercase() != "y" {
            break;
        }
    }
}
