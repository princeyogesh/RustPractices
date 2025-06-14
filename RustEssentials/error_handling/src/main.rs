/*
    Rust has Several feature to handle runtime errors
    Types of error Recoverable and unrecoverable
    Recoverable : Example file not found 
    Unrecoverable: Index beyond array bound 

    most languages do not distinguish  types of error and use exception to handle these
    .
    Rust does not have Exception,
    instead it have 
    Result<T, E> macro for Recoverable error and 
    Panic macro for unrecoverable error panic!

    --------------------
    panic! macro , immediately terminates the program, and provide feedback to caller about what went wrong
    RUST_BACKTRACE=full ENV variable will give full backtrace in rust for panics
    */
fn main() {

    //panic!(" Houstan we had a problem");  intentionally calling panic
    let countedown = [5 , 4, 3, 2, 1, 0];

    for count in countedown.iter() {
        println!("T-minus {}", count);
        let x= 1 / count;// this won't end well, will auto panic at 0
    }
    println!("Hello, world!");
}
