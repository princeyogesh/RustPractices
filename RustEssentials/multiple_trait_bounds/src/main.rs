use core::fmt;

//when we call from(b) it consumes variable b,affectively moving that variable to type T for equality comparison, that means when we use line7 it will be error
// solution data type of B needs to implement copy trait 
//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a:T, b:U) 
    where T: fmt::Display + PartialEq + From<U>, 
          U: fmt::Display + PartialEq + Copy
{
        if a == T::from(b) {
            println!("{} is equal to {} ", a, b);
        } else {
            println!("{} is not equal to {}", a, b);
        }
}

fn main() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
   // compare_and_print(1.1, "one"); will throw error
}
