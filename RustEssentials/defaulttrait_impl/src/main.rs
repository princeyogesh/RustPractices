struct Satelite{
    name:String,
    velocity:f64
}
struct SpaceStation{
    name:String,
    altitude:u32,
    crew_size:u8
}

struct SpaceShip {
    name:String,
    propllent:f64
}
trait Description {
    fn describe(&self) -> String {
        return String::from("an object flying through space");
    }
}

impl Description for Satelite {
    fn describe(&self) -> String {
        return format!("the {} flying at {}", self.name, self.velocity);
    }
}
impl Description for SpaceStation {
    fn describe(&self) -> String {
        return format!("the space station {} is at {} carrying {} people", self.name, self.altitude, self.crew_size);
    }
}

//Default trait implementation
impl Description for SpaceShip {
    
}

fn main() {
   let sat1 = Satelite{
    name:String::from("Hubble"),
    velocity:4.56
   };
   let iss = SpaceStation{
    name:String::from("International space statiosn"),
    altitude:123,
    crew_size:6

   };
   println!("Sat is {}", sat1.describe());
   println!("iss is {}", iss.describe());

   let ship = SpaceShip{
    name:String::from("chandrayan1"),
    propllent: 53.32
   };
   println!("ship is {}", ship.describe() );
}
