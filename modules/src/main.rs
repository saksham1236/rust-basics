use std::io;
use rand::prelude::*;
fn main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    //Used to read from the standard input
    io::stdin().read_line(&mut buffer);
    println!("Buffer is {}", buffer);

    let number = buffer.trim().parse::<i32>().unwrap();
    let number:i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
    // let number_rand = rand::random::<f64>();
    let number_rand = thread_rng().gen_range(1..11);
    println!("number is {}", number_rand);

}
