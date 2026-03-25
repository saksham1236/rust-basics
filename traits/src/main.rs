#[derive(PartialEq, PartialOrd)] //Deriving default traits and PartialOrd compares name in terms of sizes
struct Satellite {
    name: String,
    velocity: f64
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32
}

//Trait described seprately
trait Description {
    fn describe(&self) -> String {
        String::from("A space saucer") //Default implementation
    }
}

impl Description for Satellite {
    // fn describe(&self) -> String {
    //     format!("the {} flying at {} miles",self.name, self.velocity)
    // }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles high with {} people",self.name, self.altitude, self.crew_size)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("GPS1"),
        velocity: 2.42
    };
    let gps = Satellite {
        name: String::from("GPS1"),
        velocity: 2.42
    };

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 243
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());

    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}
