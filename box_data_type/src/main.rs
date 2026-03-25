use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}
fn main() {
    let mut vehicle = Shuttle {
        name: String::from("E"),
        crew_size: 7,
        propellant: 835958.0
    };

    println!("size: {} bytes",mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);

    println!("size: {} bytes",mem::size_of_val(&boxed_vehicle));
    println!("size on heap: {} bytes",mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle; //Dereference operator can be used to move data back from heap to Stack
    println!("size: {} bytes",mem::size_of_val(&unboxed_vehicle));
}
