
/*
    Define enum named Location
    Location can be 
        Unknonwn, Anonyms, Known - with longitude and lattitude stored as float values.
    
    implement display() metheod to print location info
    output should be different based on variant
*/
enum Location {
    Unknonwn,
    Anonyms,
    Known(f64, f64)
}
impl Location {
    fn display(&self) {
        match self  {
            Location::Anonyms => println!(" Location is Anonyms at this moment"),
            Location::Unknonwn => println!(" Location is Unknown at this moment"),
            Location::Known(lat,long ) => println!("Location is lat {} and long {}", lat, long),
            _ => println!(" none of Locationtype")
        };
    }
}
fn main() {
    let address = Location::Unknonwn;
    address.display();
    let address = Location::Anonyms;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
