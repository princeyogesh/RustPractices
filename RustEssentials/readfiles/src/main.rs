use std::fs;
fn main() {
    //read as lines
    let content = fs::read_to_string("planet.txt").unwrap();
   // println!("contet is {}", content);

    for line in content.lines() {
        println!("line is {}", line);
    }

    //reading as bytes
    let content = fs::read("planet.txt").unwrap();
    println!("content is {:?}", content);
}
