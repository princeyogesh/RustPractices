/*
    PASSING CREDENTIALS via ENVIRONMENT VARIABLE
*/
fn third_party_api(username:&str, password:&str) -> String {
    let mut accesstoken = username.to_string();
    accesstoken.push_str(password.split_at(2).0);
    
    accesstoken
}
fn main() {
    let username = "Elton".to_string();     //hardcoded username
    let password = "my_password".to_string();   //hardcoded password
    //use env to compile time or runtime
    //-----------------------------------------------------------------------
    //Accessing environment variable at compile time use env! or option_env! macro
    //----------------------------------------------------------------------
    //
    let username = env!("USERNAME");
    //let password = env!("PASSWORD");    error better use option_env
    let password = option_env!("PASSWORD");
    println!("Access token: {}", third_party_api(&username, &password))
}
