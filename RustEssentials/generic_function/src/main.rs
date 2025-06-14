fn get_bigger<T: PartialOrd>(a:T, b:T) -> T {
    if a > b {
        a
    } else {
        b
    }

    
}
fn main() {
    println!("biggest is {}", get_bigger(1.3, 2.5));
    println!("biggest is {}", get_bigger(3, 2));
}
