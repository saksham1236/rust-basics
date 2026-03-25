use std::fmt;
use std::fmt::Formatter;
use std::cmp::Ordering;
struct Satellite {
    name: String,
    velocity: f64 //Miles /s
}

impl fmt::Display for Satellite {
 fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
     write!(f, "{} is the name {} is the velocity", self.name, self.velocity)
 }
}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.velocity.partial_cmp(&other.velocity)
    }
}

impl PartialEq for Satellite {
    fn eq(&self, other: &Self) -> bool {
        self.velocity == other.velocity
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    let stubble = Satellite {
        name: String::from("Stubble"),
        velocity: 2.7
    };

    println!("hubble is {}", hubble);
    println!("hubble is faster than stubble {}", hubble > stubble);
}
