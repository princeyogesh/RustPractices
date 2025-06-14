/*
    in struct elements are named, in tuple element are unnamed(you have to remember order of element in tuple)
*/

#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellent: f64
}
fn main() {
    println!("Hello, world!");
    let mut vehicle = Shuttle{
        name:String::from("Endavor"),
        crew_size:7,
        propellent: 83456.234
    };
    vehicle.crew_size += 2;
    println!("name is {}", vehicle.name);
    println!("vehicle is {:?}", vehicle);

    let vehicle2 = Shuttle {
        ..vehicle.clone();
    };
    println!("vehicle is {:?}", vehicle);
    vehicle.propellent = 12.0;
    println!("vehicle is {:?}", vehicle2);
}
