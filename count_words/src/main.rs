use std::collections::HashMap;
use std::env;
use std::{fs, io};

fn count_words() -> (Vec<String>, u32) {
    if env::args().len() <= 1 {
        println!("Not enough arguements restart and put in a file path");
    }

    let file_path = env::args().nth(1).expect("Arguement error");
    let file_content = match fs::read_to_string(file_path) {
        Ok(s) => s.to_lowercase(),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),
            _ => panic!("Unknown error: {:?}", e)
        }
    };

    let words = file_content.split_whitespace();
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    let mut max_count:u32 = 0;
    let mut max_count_words:Vec<String> = Vec::new();
    for word in words {
        let count = word_count.entry(&word).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            max_count_words.push(String::from(word));
        }

        if *count == max_count {
            max_count_words.push(String::from(word));
        }
    }
    
    (max_count_words, max_count)


}

fn main() {
    let (word, count) = count_words();
    println!("Max words are {:?} and it appears number of time {}", word, count);
}
