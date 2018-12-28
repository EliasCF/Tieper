pub use db::SqlHandler;

pub struct StopStrategy {}

impl StopStrategy {
    pub fn handle (id: i32) {
        println!("{}", id);

        let mut handler = SqlHandler::new();

        match handler.update(id, false) {
            Ok(v) => println!("{}", v),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}