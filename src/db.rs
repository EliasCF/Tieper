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

        match self.connection.execute(query) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn delete (&mut self, opts: SingleId) -> Result<(), ()> {
        let query = 
            format!(
                "DELETE FROM keepers
                 WHERE Id = {}",
                 opts.id
            );
        
        match self.connection.execute(query) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn update (&mut self, opts: SingleId, value: bool) -> Result<(), ()> {
        let query = 
            format!(
                "UPDATE keepers
                 SET Active = {}
                 WHERE Id = {}",
                 value,
                 opts.id
            );

        match self.connection.execute(query) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn select (&mut self, opts: ListOpts) -> Result<(), ()> {
        let mut where_clause = String::from("WHERE ");

        let mut name_flag = false;

        match opts.name {
            Some(name) => {
                where_clause += format!("Name = '{}' ", name).as_str();
                name_flag = true;
            },
            None => ()
        }

        if opts.active && !opts.inactive {
            if name_flag {
                where_clause += "AND ";
            }

            where_clause += "Active = true";
        }

        if !opts.active && opts.inactive {
            if name_flag {
                where_clause += "AND ";
            }

            where_clause += "Active = false";
        }

        if where_clause == "WHERE " { where_clause = String::new() }

        let query_reslut =
            self.connection.iterate(format!("SELECT * FROM keepers {}", where_clause), |paris| {
                for &(column, value) in paris.iter() {
                    println!("{} = {}", column, value.unwrap());
                }
                true
            });

        match query_reslut {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}