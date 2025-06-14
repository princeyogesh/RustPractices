use std::collections::HashMap;

/*
    HashMap <K,V> DataType

    order of key may not same as you interested
*/
fn main() {

    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadefield", 3);
    missions_flown.insert("hurley", 3);
    missions_flown.insert("berron", 0);
   // missions_flown.insert(Barrod, v)

   print!("missions_flown are {:?} ", missions_flown);

   let barron_missions = missions_flown.get("berron");
   println!("berron mission is {:?}", barron_missions);
   //
   missions_flown.insert("berron", 1);
   let barron_missions = missions_flown.get("berron");
   println!("berron mission is {:?}", barron_missions);
   ///
   /// 
   missions_flown.entry("berron").or_insert(5);
   let barron_missions = missions_flown.get("berron");
   println!("berron mission is {:?}", barron_missions);
}
