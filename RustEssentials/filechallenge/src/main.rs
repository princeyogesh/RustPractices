/*
    Write a programme to checl if a specific person exist in list of names
    Print a message indicating whether list has name

    Acept two CL args
    -Path to FIle
    -Name to search in file 
*/
use std::env;
use std::fs;
fn main() {
    if std::env::args().len() < 3 {
        println!("too less arguments");
        return;
    }
    let args = std::env::args();

    for (index, arg) in args.enumerate(){
        println!("arg number {} is {} ", index, arg);
    }
    let filename = std::env::args().nth(1).unwrap();
    let name_to_search = std::env::args().nth(2).unwrap();
    println!("file is {} and search {}", filename, name_to_search);

    let content = fs::read_to_string(filename).unwrap();

    for line in content.lines() {
        if line == name_to_search
        {
            println!("name found");
            return;
        }
    }
    println!("Name not found");
}
