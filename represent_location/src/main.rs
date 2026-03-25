use core::fmt;
use std::fmt::{Display, Formatter};

enum Location{
    Unknown,
    Anonymous,
    Known(f64, f64)
    // Known {latitude:f64, longitude:f64 } enums can also contain structs
}

impl Location {
    fn display(&self) {
        match self{
            Location::Unknown => {println!("Location is unknown");}
            Location::Anonymous => {println!("Location is anonymous")}
            Location::Known(latitude, longitude) => {println!("Location is {}, {}", latitude, longitude)}
        }
    }
}

impl fmt::Display for Location{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Location::Anonymous => {write!(f, "Location is An")}
            Location::Unknown => {write!(f, "Location is Un")}
            Location::Known(latitude, longitude) => {write!(f, "Location is {}, {}", latitude, longitude)}
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
    println!("{}", address);
}
