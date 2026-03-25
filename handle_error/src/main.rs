use std::io;
use rand::prelude::*;

fn main() {
    higher_lower();
}

fn higher_lower(){
    let number_rand = thread_rng().gen_range(1..100);
    let mut input_num:u8 = input_number("Enter a number");
    let mut guesses:u16 = 0;
    while input_num != number_rand {
        if input_num > number_rand {
            input_num = input_number("Number is smaller");
            guesses += 1;
        } else if input_num < number_rand {
            input_num = input_number("Number is greater");
            guesses += 1;
        }
    }
    println!("number was {} in {} guesses", number_rand, guesses);
}

fn input_number(message: &str) -> u8 {
    println!("{}", message);
    // let mut buffer = String::new();
    // let input = io::stdin().read_line(&mut buffer);
    // match input {
    //     Ok(s) => s as u8,
        
    // }
    loop {
        let mut buffer = String::new();
        let input = io::stdin().read_line(&mut buffer);
        match input {
            Ok(_) => match buffer.trim().parse::<u8>() {
                Ok(value) => return value,
                Err(_) => {
                    println!("Failed to parse input, Try Again");
                    continue
                }
            }
            Err(_) => {
                println!("Failed to read input try again");
                continue
            }
        }
    }
}