use std::any::type_name;

struct Shuttle {
    name:String, 
    crew_size:u8,
    propellent:f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn add_fuel(&mut self, gallon:f64) {
            self.propellent += gallon;
    }
}
fn main() {
    println!("Hello, world!");
    let mut vehicle = Shuttle {
        name:String::from("Endavour"),
        crew_size : 7,
        propellent : 12312.13
    };
    let vehicle_name = vehicle.get_name();
    println!("name os {}", vehicle_name);

    println!("propellent level is {}", vehicle.propellent);
    vehicle.add_fuel(100.1);
    println!("propellent level is {}", vehicle.propellent);
}
