/*
    'static lifetime------
        - Indicates refrences availiable for entire duration of the program, data never get dropped
        - Example string literals
                  let s: &'static str = "Greetings from Neptune";
        -'static lifetime as a Trait Bound
            ensures datatype will only contain 'static elements
            T:Display + 'static
        
*/
fn main() {
    println!("Hello, world!");
}
