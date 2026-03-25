fn best_fuel<'a, 'b>(x: &'a str, y:&'b str) -> &'a str { //Borrow checker checks the relative lifetimes of the input parameters to the result and will use the shorter one
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

struct Shuttle<'a> {
    name: &'a str
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'b self, msg: &'a str) -> &'a str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);

    let vehicle = Shuttle {
        name: "Endeavour"
    };

    let sender = vehicle.send_transmission("Greetings");
    println!("sender is {}",sender);
}


