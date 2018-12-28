extern crate mysql;
use self::mysql::params;

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


    pub fn insert (&mut self, inactive: bool, name: String) -> Result<(), ()> {
        let query_result =
            self.connection.prep_exec(
                "INSERT INTO keepers
                    (Name, Active, ElapsedTime, OverallTime)
                VALUES
                    (:name, :active, :elapsedtime, :overalltime)",
                params!{
                    "name" => name, 
                    "active" => !inactive,
                    "elapsedtime" => chrono::Duration::zero(),
                    "overalltime" => chrono::Duration::zero(),
                }
            );

        match query_result {
            Ok(_v) => Ok(()),
            Err(_e) => Err(())
        }
    }

    pub fn delete (&mut self, id: i32) -> Result<String, ()> {
        let query_result =
            self.connection.prep_exec(
                "DELETE FROM keepers 
                 WHERE Id = :id",
                params!{"id" => id}
            );

        match query_result {
            Ok(v) => {
                if v.affected_rows() == 0 {
                    return Ok(String::from(
                        format!("Keeper id {}, was not found", id)
                    ))
                }

                return Ok(String::from("Success"))
            },
            Err(_e) => Err(())
        }
    }

    pub fn update (&mut self, id: i32, value: bool) -> Result<String, ()> {
        let query_result =
            self.connection.prep_exec(
                "UPDATE keepers
                 SET Active = :active
                 WHERE Id = :id",
                params!{
                    "active" => value,
                    "id" => id,
                }
            );

        match query_result {
            Ok(v) => {
                if v.affected_rows() == 0 {
                    return Ok(String::from(
                        format!("Keeper id {}, was not found", id)
                    ))
                }

                return Ok(String::from("Success"))
            },
            Err(_e) => Err(())
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