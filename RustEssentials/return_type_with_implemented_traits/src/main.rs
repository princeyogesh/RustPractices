use std::fmt;

fn get_displayable(choice: bool) -> impl fmt::Display { //retrun something that implement Display trait

    //"thirteen"    works
    if choice {
       return  13;
    } /*else {
        "thirteen"          //uncommenting this block will throw error though "thiteen" amd int(13) implement Display trait but within same function rust finds this ambiguos
    }   */
   36    
}
fn main() {
    println!("output is {}", get_displayable(true));

}
