pub use db::SqlHandler;

pub struct RemoveStrategy {}

impl RemoveStrategy {
    pub fn handle (id: i32) {
        println!("{}", id);

        let mut handler = SqlHandler::new();
        
        match handler.delete(id) {
            Ok(v) => println!("{}", v),
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}