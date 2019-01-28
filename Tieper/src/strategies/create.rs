pub use db::SqlHandler;
pub use commands::CreateOpts;

pub struct CreateStrategy {}

impl CreateStrategy {
    pub fn handle (opts: CreateOpts)  {
        let mut handler = SqlHandler::new();

        match handler.insert(opts) {
            Ok(_) => println!("Success!"),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}