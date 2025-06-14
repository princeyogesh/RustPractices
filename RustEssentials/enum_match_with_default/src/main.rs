use std::result;

fn main() {
    let my_number = 1u8;

    let reuslt = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match ", my_number);
            "default case wildcard match "
        }
        
    };
    println!("result is {}", reuslt);
}
