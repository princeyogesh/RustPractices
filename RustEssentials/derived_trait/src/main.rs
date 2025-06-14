//default dervie PartialEq returns true if all fields of both struct are exactly same
//PartialOrd compares attributes of struct , which is greater
#[derive(PartialEq, PartialOrd )]
struct Satelite {
    name:String,
    velocity:f64
}
fn main() {
    println!("Hello, world!");
    let hubble = Satelite {
        name: String::from("hubble Telescope"),
        velocity: 5.45
    };
    let gps = Satelite {
        name:String::from("GPS"),
        velocity: 1.2
    };

    println!("hubble == gps is {}", hubble==gps);
    println!("hubble > gps is {}", hubble>gps);
}
