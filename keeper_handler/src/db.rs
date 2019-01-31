extern crate sqlite;

use std::path::Path;
use std::{thread, time};

pub struct SqlHandler {
    connection: sqlite::Connection,
}

impl SqlHandler {
    pub fn new () -> SqlHandler {
        match sqlite::open(Path::new("../../keeperdb.db")) {
            Ok(conn) => {
                conn.execute(
                    "CREATE TABLE IF NOT EXISTS keepers (
                         Id INTEGER PRIMARY KEY AUTOINCREMENT,
                         Name VARCHAR(50) NOT NULL,
                         Active BOOLEAN NOT NULL,
                         ElapsedTime VARCHAR(50) NOT NULL,
                         OverallTime VARCHAR(50) NOT NULL
                     )").unwrap();

                thread::sleep(time::Duration::from_millis(10000));

                SqlHandler {
                    connection: conn
                }
            },
            Err(error) => {
                panic!("{}", error);
            }
        }
    }
}