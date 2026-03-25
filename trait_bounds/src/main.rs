use std::any;
use std::fmt;

fn print_type<T: fmt::Debug>(item: T){
    println!("{:?} is {}", item, any::type_name::<T>())
}
//
//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) //Traits can be added to variables
//Or using the where clause
fn compare_and_print<T, U>(a: T, b: U)
    where T: fmt::Display + PartialEq + From<U>, 
        U: fmt::Display + PartialEq + Copy
    {
    if a == T::from(b){
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b)
    }
}
//Traits for return value
fn get_displayable(choice: bool) -> impl fmt::Display {
    if choice {
        13
    } else {
        "13" //Rust does static dispacth using monomorphization, Here dynamic dispatch is required
    }
}

fn main() {
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);

    compare_and_print(1.0, 1);
    // compare_and_print(1.1, "one");

    println!("output is {}", get_displayable(true))
}

