pub use db::SqlHandler;

pub struct StartStrategy {}

impl StartStrategy {
    pub fn handle (id: i32) {
        println!("{}", id);

        let mut handler = SqlHandler::new();

        match handler.update(id, true) {
            Ok(v) => println!("{}", v),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}