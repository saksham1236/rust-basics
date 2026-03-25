//Structs are only structures
#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}
//Methods can use data from Structs
impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 0,
            propellant: 0.0
        }
    }
}
fn main() {
    //Need to be defined like typescript types and have instances created
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 85848.0
    };

    let vehicle_name = vehicle.get_name(); 
    println!("vehicle name {vehicle_name}");

    println!("Add fuel {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("Add fuel {}", vehicle.propellant);

    // let vehicle2 = Shuttle {
    //     name: String::from("Discovery"),
    //     ..vehicle.clone()
    // };

    // vehicle.name = String::from("Atlantis");
    // println!("vehicle is {:?}", vehicle);
    // println!("vehicle is {:?}", vehicle2);
    
}
