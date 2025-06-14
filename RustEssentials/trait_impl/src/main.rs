
struct Satelite {
    name: String,
    velocity: f64
}
struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32
}

trait Description {
    fn describe(&self) ->String;
}

impl Description for Satelite {
    fn describe(&self) ->String {
        return format!("the {} flying at {} miles per second", self.name, self.velocity);
    }
}
impl  Description for SpaceStation {
    fn describe(&self) ->String {
        return format!("the {} flying {} miles high, {} crew_members onboard", self.name, self.altitude, self.crew_size);
    }
}
fn main() {
    println!("Hello, world!");

    let hubble = Satelite{
        name:String::from("Hubble Telescope"),
        velocity:4.72
    };
    let iss = SpaceStation {
        name: String::from("International Space station"),
        crew_size: 6,
        altitude:254
    };

    println!("hubble is {}", hubble.describe()); //default println will not print since not trait is implementd to format struct
    println!("iss is {}", iss.describe());
}
