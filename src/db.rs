extern crate mysql;

extern crate chrono;

pub struct SqlHandler {
    connection: mysql::Conn,
}

impl SqlHandler {
    pub fn new () -> SqlHandler {
        match mysql::Conn::new("mysql://root:@localhost:3306/keepersdb") {
            Ok(mut connection) => {
                //Make sure that the table 'keepers' exists
                let selected: Vec<String> = 
                    connection.prep_exec("SHOW TABLES LIKE 'keepers'", ()).map(|result| {
                        result.map(|x| x.unwrap()).map(|row| {
                            mysql::from_row(row)
                        }).collect()
                    }).unwrap();

                //Generate a table if no table was found
                if selected.len() < 1 {
                    connection.prep_exec(
                        r"CREATE TABLE keepers (
                            Id INT NOT NULL PRIMARY KEY,
                            Name VARCHAR(100) NOT NULL,
                            Active BOOLEAN NOT NULL,
                            ElapsedTime TIME NOT NULL,
                            OverallTime TIME NOT NULL
                        );", ()
                    ).unwrap();
                }

                SqlHandler {
                    connection: connection,
                }
            },
            Err(error) => {
                panic!("The program was unable to connect to the database: {}", error)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Keeper {
    id: i32,
    name: String,
    active: bool,
    elapsed_time: chrono::Duration,
    overall_time: chrono::Duration,
}