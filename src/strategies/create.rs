pub use db::SqlHandler;

pub struct CreateStrategy {}

impl CreateStrategy {
    pub fn handle (inactive: bool, name: String)  {
        println!("{}, {}", inactive, name);

        let mut handler = SqlHandler::new();

        match handler.insert(inactive, name) {
            Ok(_) => println!("Success!"),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}