use std::fmt;

/*
    Implement the Dispay Trait for custom struct
    Implement the PartialOrd trait to compare stellites 
   
*/

struct Satellite {
    name:String, 
    velocity: f64
}
impl Satellite {
    fn new(new_name: String, new_velo: f64) -> Satellite {
        Satellite { name: (new_name), velocity: (new_velo) }
    }
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "this satellite is {} and flying at {} miles per hr", self.name, self.velocity)
    }
    
    //fmt!("the satelite is named as {} and velocity {}", Satellite::name, Satellite::velocity)
}

impl PartialEq for Satellite  {
    fn eq(&self, other: &Self) -> bool {
        if self.name == other.name && self.velocity == other.velocity {
            return true;
        }
        return false;
    }
    fn ne(&self, other: &Self) -> bool {
        if self.name != other.name || self.velocity != other.velocity {
            return true;
        }
        return false;
    }
}
impl PartialOrd for Satellite  {
    fn ge(&self, other: &Self) -> bool {
        return self.velocity >= other.velocity;
    }
    fn gt(&self, other: &Self) -> bool {
        return self.velocity > other.velocity;
    }
    fn le(&self, other: &Self) -> bool {
        return  self.velocity <= other.velocity;
        
    }
    fn lt(&self, other: &Self) -> bool {
        return  self.velocity < other.velocity;
        
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.velocity < other.velocity {
            return Some(std::cmp::Ordering::Less);
        } else if self.velocity > other.velocity {
            return Some(std::cmp::Ordering::Greater);
        } else if self.velocity == other.velocity {
            return Some(std::cmp::Ordering::Equal);
        }
        return Some(std::cmp::Ordering::Equal);
    }
}
fn main() {
    let hubble = Satellite::new(String::from("hubble"), 15.34);

    println!("hubble is {}", hubble);
}
