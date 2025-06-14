use std::{collections::HashMap, fs};

/*
    Goal : 
    read a text file
        accept path as Command-line argument.
        parse text into individual words 
        split_whitespaces();

        keep track og how many times each unique word occues
    Count number of time each word occurs

    Print a message with most common words and how many times they appeared
    . 
 */

 fn process_file(filename: &str) {
     println!("processing file {}", filename);
     let content = fs::read_to_string(filename).unwrap().to_lowercase();
     println!("content is {}", content);
     let mut mp: HashMap<&str, i32> = Default::default();
     for word in content.split_whitespace(){
        if mp.contains_key(word) {
            mp.insert(word, mp.get(word).unwrap_or(&0) + 1);
        } else  {
            mp.insert(word, 0);
        }
     }

     let (most_frequent_word, frequent_count) = find_frequent(mp);
     println!("*******************************************************");
     println!("most frequent word is {} and it appeared {} times ", most_frequent_word, frequent_count);

 }

 fn find_frequent(mp: HashMap<&str, i32>) ->(String, i32) {
     let mut  max_count = 0;
     let mut ans:String = Default::default();
     for (s, count) in mp {
         if count > max_count {
            max_count = count;
            ans = String::from(s);
         }
     }
     return (ans, max_count);
 }
fn main() {
    let env_vars = std::env::args();
    if env_vars.len() < 2 {
        panic!(" too less arguments please provide file name to be scanned");
    }
   let aargs: Vec<String> = env_vars.collect();
   let filename = &aargs[1];
   println!("filename is {}", filename);
   ////////////////////////////////////////////////////////////
   
   process_file(filename);



}
