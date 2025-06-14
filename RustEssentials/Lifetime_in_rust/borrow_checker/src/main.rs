/*
    under the hood rust compiler uses BorrowChecker, which compares the scope of variables
    in scope.
    Borrow checker will determine the lifetime of variable     
 */
fn main() {

    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        println!("propellant is {}", propellant);
    }
    println!("propellant is {}", propellant);       //problem rp1 is out of scp[e bprrpwed by propellent]
}
