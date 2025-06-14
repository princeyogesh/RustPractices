fn main() {
   
   let num = Some(13);
   match num {
       Some(13) => println!("thirteen"),
       _ => ()
   }
   /////////////////////////////////////////////////
   /// Simpler syntax //////////////////////////////
   if let Some(13) = num {
    println!("Matched thirteen")
   }
}
