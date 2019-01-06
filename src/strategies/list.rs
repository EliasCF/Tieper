pub use db::SqlHandler;
pub use commands::ListOpts;

extern crate chrono;

pub struct ListStrategy {}

impl ListStrategy {
    pub fn handle (opts: ListOpts) {
        let mut handler = SqlHandler::new();

        match handler.select(opts) {
            Ok(_v) => {
            },
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}