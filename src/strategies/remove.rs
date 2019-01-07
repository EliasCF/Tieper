pub use db::SqlHandler;
pub use commands::SingleId;

pub struct RemoveStrategy {}

impl RemoveStrategy {
    pub fn handle (opts: SingleId) {
        let mut handler = SqlHandler::new();
        
        match handler.delete(opts) {
            Ok(_) => (),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}