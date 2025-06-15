/*
    Package contains one or more binary/library crate
    here 
    packagename by default is PackageVsCrates (root directory)
    we can see packagename in cargo.toml
    .
    how does cargo knows what to compile and build??????
        - It bydefault sees src folder and main.rs in file
        - after compilation cargo creates executable same name as package.
        --cargo new --lib will give , lib.rs in src folder instead of main.rs
        - when cargo sees lib.rs  file it knows that root file to compile a library crate.
          compiler doesnot expect main function in lib.rs
        - if src has both files lib.rs and main.rs , two sepearate crates are there.
*/
fn main() {
    println!("Hello, world!");
}
