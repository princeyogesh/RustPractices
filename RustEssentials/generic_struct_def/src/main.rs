use std::sync::Arc;

/*
    Generics are Zero -Cost Abstraction
    Make programming Easier without reducing runtime performance

    COmpiler uses process called monomortizations
    Compiler replaces generic placeholders with concrete datatypes
*/
struct Rectangle<T> {
    width: T,
    height: T
}

impl<T> Rectangle<T> {
    
    fn get_width(&self) -> &T {
       return  &self.width;
    }   //returning refrencessince we do not know at runtime whether T will be on heap or stack
}

impl Rectangle<u8> {
    fn get_area(&self) ->  u8 {
        return (self.width * self.height) ;
    }
}
fn main() {
    let rect = Rectangle{
        width:1u8,
        height:2u8
    };
    
    println!("width is {} and area is {}", rect.get_width(), rect.get_area());
}
 