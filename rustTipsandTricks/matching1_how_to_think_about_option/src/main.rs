#[derive(Debug)]
struct Invitation {
    invitee:String, 
    attending:bool,
    message:Option<String>,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message:Option<String>) ->Invitation {
        Invitation{
            invitee,
            attending,
            message
        }
    }
}
fn main() {
    let invition_1 = Invitation::new("DOlares".to_string(), true, None);
    let invitation_2 = Invitation::new("Cosmo".to_string(), false, Some("PLanning to go mars this week".to_string()));

    println!("{:#?}", invition_1);
    println!("{:#?}", invitation_2);
}
