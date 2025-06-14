/*when struct  stores data using Strign data type, when struct 
goes out of scope String data also deallocates from heap */
/* 
struct Shuttle
{
    name: String
}
*/
/*
    in below implementation since struct does not own the refrence of &str name variable.
    we have to explicitly define lifetime of struct and &str since (it is refrences)
    .
    how lifetime of borrowed string relaetes to lifetime of struct is unclear.
    if string is dropped then disappears while struct is still in scope,
    then struct tries to refrence and use no longer existing string.
    so need lifetime
*/
struct Shuttle<'a>
{
    name: &'a str
}

impl<'a> Shuttle<'a> {
    fn send_transmission(&self, msg: &str) -> &str {
        println!("Transmitting the msg : {}", msg);
        return  self.name;
    }
}
fn main() {
    let vehicle = Shuttle{
        name: "Endabor"
    };
    let sender = vehicle.send_transmission("Greeting from orbit");
    println!("sender is {}", sender);
}
