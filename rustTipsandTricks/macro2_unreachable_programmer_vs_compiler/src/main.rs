fn even_or_odd(x:i32) ->String {
    match x % 2 {
        0 => "even".to_string(),
        1 | -1 => "odd".to_string(), // here there is no default case so use unreachalle macro
        _ => unreachable!()
    }
}
fn main() {
for x in -10..10 {
    dbg!(x, even_or_odd(x));
}
}
