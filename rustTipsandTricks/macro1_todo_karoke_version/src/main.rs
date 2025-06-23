/*
    Rust while implementing t
    Rust trait is abstract way to defined behaviour
*/

use std::io::Error;

trait Karoke {
    fn pick_song_i_know() ->Option<String>;
    fn sing() -> Result<(), Error>;
}
/*
    now if some struct tries to implement trait, and has no defination ready of some of its function
    todo macro is useful for indication that implementaion is yet to be done
*/

struct Bubble{}

impl Karoke for Bubble {
    fn pick_song_i_know() ->Option<String> {
       let s = String::from("Kolaveri D"); 
       return  Some(s);
    }
    fn sing() -> Result<(), Error> {
       //not implemented yet
       todo!()
    }
}
fn main() {
    

    println!("Hello, world!");
}
