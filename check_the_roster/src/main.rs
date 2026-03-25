use std::env;
use std::fs;

fn main() {
    
    if env::args().len() < 2 {
        println!("Not enough arguements");
    }

    if check_name(env::args().nth(1).unwrap(), env::args().nth(2).unwrap()) {
        println!("Name was found");
    } else {
        println!("Name wasn't found")
    };


}

fn check_name(file:String, name:String) -> bool {
    for line in fs::read_to_string(file).unwrap().lines() {
        let trimmed_line = line.trim();
        if name == trimmed_line {
            return true
        }
    }
    return false
}
