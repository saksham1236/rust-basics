#[derive(Debug)]
//Generic Struct
struct Rectangle<T, U>{
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U>{ //Impl needs generic data types for scope and retangle data types for name;
    fn get_width(&self) -> &T {
        &self.width //Needs to return a reference otherwise heap based datatypes will transfer ownership
    }
}
//Implementations for specific data types
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        (&self.height + &self.width)*2
    }
}

//Generic functions
fn get_biggest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {//Need to add std::cmp::PartialOrd to let rust know that <T> is compareable
    if a > b {a} else {b}
}
fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u8
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("Perimeter is {}", rect.get_perimeter());

    //Generic Functions
    println!("biggest is {}", get_biggest(3.4, 7.6))
}
