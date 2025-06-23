/*
    write a program that gives us the Nth
    value in following sequence 
    5,8,11,14,17
*/
fn recursive_solution(n: u64) -> u64 {
    match n  {
            0 => panic!("input must be higher than 1"),
            1=> 5,
            _ => recursive_solution(n-1) + 3
    }
}
fn main() {
    dbg!(recursive_solution(1));

}
