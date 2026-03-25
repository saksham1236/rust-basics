use std::fs;
use std::io::prelude::*;
fn main() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("{contents}");

    for line in contents.lines() {
        println!("line is {}", line)
    }

    //Read file as bytes
    let contents_bytes = fs::read("planets.txt").unwrap();
    println!("contents is {:?}", contents_bytes);

    //Write to a file
    let mut speech = String::new();
    speech.push_str("This is a string, \n");
    speech.push_str("This is not a string \n");

    //Writing to file
    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");
}
