/*
    How do you represent nothing, like null in c++, or None in python
    errors occur when you try to access no value,
    so rust has no concept of null value
    .
    rust uses new way of Options<T> enum
    Optionn<T> is included in prelude 
*/

fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(4);
    let number = number.unwrap_or(&0) + 1;       //using unwrap is discouraged , since number can be none as well
                                                              //use unwrap_or method instead 
    println!("number is {:?}", number);    
}
