use std::env;
fn main() {

    if env::args().len() <= 2 {
        println!("Not enough arguements")
    }
    for (index, arguement) in env::args().enumerate() {
        println!("arguements {} is {}", index, arguement);
    }

    let args2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", args2);
}
