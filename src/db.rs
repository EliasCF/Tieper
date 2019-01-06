extern crate sqlite;

use std::path::Path;

pub use commands::{SingleId, CreateOpts, ListOpts};

extern crate chrono;

pub struct SqlHandler {
    connection: sqlite::Connection,
}

impl SqlHandler {
    pub fn new () -> SqlHandler {
        match sqlite::open(Path::new("./keeperdb.db")) {
            Ok(conn) => {
                conn.execute(
                    "CREATE TABLE IF NOT EXISTS keepers (
                         Id INTEGER PRIMARY KEY AUTOINCREMENT,
                         Name VARCHAR(50) NOT NULL,
                         Active BOOLEAN NOT NULL,
                         ElapsedTime VARCHAR(50) NOT NULL,
                         OverallTime VARCHAR(50) NOT NULL
                     )").unwrap();

                SqlHandler {
                    connection: conn
                }
            },
            Err(error) => {
                panic!("{}", error);
            }
        }
    }

    pub fn insert (&mut self, opts: CreateOpts) -> Result<(), ()> {
        let query = 
            format!(
                "INSERT INTO keepers 
                    (Name, Active, ElapsedTime, OverallTime)
                 VALUES
                    ('{}', {}, '{}', '{}')", 
                opts.name,
                !opts.inactive,
                chrono::Duration::zero().num_seconds().to_string(),
                chrono::Duration::zero().num_seconds().to_string()
            );

        self.connection
            .execute(query)
            .unwrap();

        Ok(())
    }

    pub fn delete (&mut self, opts: SingleId) -> Result<String, ()> {
        let query = 
            format!(
                "DELETE FROM keepers
                 WHERE Id = {}",
                 opts.id
            );
        
        self.connection
            .execute(query)
            .unwrap();

        Ok(String::new())
    }

    pub fn update (&mut self, opts: SingleId, value: bool) -> Result<String, ()> {
        let query = 
            format!(
                "UPDATE keepers
                 SET Active = {}
                 WHERE Id = {}",
                 value,
                 opts.id
            );

        self.connection
            .execute(query)
            .unwrap();

        Ok(String::new())
    }

    pub fn select (&mut self, opts: ListOpts) -> Result<Vec<String>, ()> {
        self.connection.iterate("SELECT * FROM keepers", |paris| {
            for &(column, value) in paris.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();

        Ok(Vec::new())
    }
}