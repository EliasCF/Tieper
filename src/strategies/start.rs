pub use db::SqlHandler;
pub use commands::SingleId;

pub struct StartStrategy {}

impl StartStrategy {
    pub fn handle (opts: SingleId) {
        let mut handler = SqlHandler::new();

        match handler.update(opts, true) {
            Ok(v) => println!("{}", v),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}