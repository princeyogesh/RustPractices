use std::fs;

/*
    Recoverable errors,
    Errors that do not cause programme to fail and can be corrected

    Example : File not found, we can resolve it by proper handling 
    to facilate recoverable error

    rust std library gives enum with two varianlt
    Result<T,E> {
        OK(T) ,
        Err(E)
    }
*/
fn main() {
    let contents = fs:: read_to_string("the_ultimate_questiosn.txt");
    
    println!("contents are {:?}", contents.expect("no body knows this file"));

}
