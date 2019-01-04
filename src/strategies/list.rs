pub use db::SqlHandler;
pub use commands::ListOpts;

extern crate chrono;

pub struct ListStrategy {}

impl ListStrategy {
    pub fn handle (opts: ListOpts) {
        let mut handler = SqlHandler::new();

        match handler.select(opts) {
            Ok(v) => {
                for row in v {
                    println!("Id: {} \r\nName: {} \r\nActive: {} \r\nElapsedTime: {} \r\nOverallTime: {}\r\n\r\n", 
                        row.get::<i32, usize>(0).unwrap(),
                        row.get::<String, usize>(1).unwrap(),
                        row.get::<bool, usize>(2).unwrap(),
                        row.get::<chrono::Duration, usize>(3).unwrap().num_seconds(),
                        row.get::<chrono::Duration, usize>(4).unwrap().num_seconds(),
                    );
                }
            },
            Err(_) => println!("The sql transaction was not successful"),
        }
    }
}