/*
    Vec<T> dataType
    Arrays are tored on stack, 
    vector are on heap (need to handle ownership and borrwoing)
    size of vector is dynamic to runtime
*/
fn main() {
    println!("Hello, world!");
    let mut astrpmauts: Vec<String> = Vec::new();
    astrpmauts.push( String::from("Shepard") );
    astrpmauts.push(String::from("Grission") );
    astrpmauts.push(String::from("Glenn"));

    println!("astronauts is {:?}", astrpmauts);
    //access elements of vector 
    //let third = astrpmauts[3];  //since vector stores the value it storing , we won't able to transfer ownership here 
     let third = &astrpmauts[2];    //coorect way
     println!("third is {}", third);

     let last = astrpmauts.pop();
     println!("last is {:?}", last);
}  
