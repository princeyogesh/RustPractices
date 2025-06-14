use std::fs;
use std::io::prelude::*;
fn main() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the moon in this decade\n");
    speech.push_str("and sdf things \n");
    speech.push_str("NOt because they are easy\n");
    speech.push_str("but because they are hard\n");    
    //read one shot, and overwrite whole file
    let _ = fs::write("speech.txt", speech);
    let mut file = fs::OpenOptions::new().append(true).open("speech.txt").unwrap();
    let _ = file.write(b"\nKing is Modi"); //write simply writes bytes data
}
