pub use db::SqlHandler;
pub use commands::SingleId;

pub struct StopStrategy {}

impl StopStrategy {
    pub fn handle (opts: SingleId) {
        let mut handler = SqlHandler::new();

        match handler.update(opts, false) {
            Ok(v) => println!("{}", v),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}