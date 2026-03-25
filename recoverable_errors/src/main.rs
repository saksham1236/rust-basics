use std::fs;
use std::io;

fn main() {
    let result = fs::read_to_string("texrt.txt");
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not foundue"),
            io::ErrorKind::PermissionDenied => String::from("Let me in, Let me in"),
            _ => panic!("Unknown brokei software:{:?}", error)
        }
    };
    println!("contents is {:?}", contents);
}
