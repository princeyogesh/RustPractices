/*
    rust std library includes Box<T>
    Box allows to store data on heap, instead of stack
    Box are considered smartPointer, they provide additional functionality beyond refrence
    Box<T> has ownership of data it points to
    when Box<T> goes out of scope it deallocates the heap memory


    Usecases for Box<T>
    store type whose size cannot be known at compile time
    Example recursive types

    Transfer of ownership of data rather than copt it on the stack
    Avoid copy of large amount of dataval
*/

use std::mem;

struct Shuttle {
    name :String,
    crew_size :u8,
    propplent:f64
}
fn main() {
    println!("Hello, world!");
    let vehicle = Shuttle{
        name: String::from("Endavour"),
        crew_size:4,
        propplent:53.34
    };

    println!("vehicle size on stack : {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    //vehicle variable now invalid, as it moved to boxed_vehicle

    println!("boxed vehicle size on stack : {} bytes", mem::size_of_val(&boxed_vehicle)); //data on stack
    println!("boxed vehicle size on heap : {} bytes", mem::size_of_val(&*boxed_vehicle)); //data on heap

    //
    let unboxed_vehicle:Shuttle = *boxed_vehicle;
    print!("unboxed size on stack {} bytes", mem::size_of_val(&unboxed_vehicle));  
}
