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

    pub fn select (&mut self, active: bool, inactive: bool, name: Option<String>) -> Result<Vec<mysql::Row>, ()> {
        let mut where_clause = String::from("WHERE ");

        let mut parameters: Vec<(String, mysql::Value)> = Vec::new();

        //This flag indicates whether the 'name' parameter is None or Some
        let mut name_flag = false;

        /*
         * Query building
         */
        match name {
            Some(value) => {
                 parameters.push((String::from("name"), mysql::Value::Bytes(Vec::from(value))));
                 where_clause += " Name = :name ";
                 name_flag = true;
            },
            None => (),
        }

        if active && !inactive {
            if name_flag {
                where_clause += "AND ";
            }

            where_clause += "Active = :active";
            parameters.push((String::from("active"), mysql::Value::Int(1)));
        }

        if !active && inactive {
            if name_flag {
                where_clause += "AND ";
            }

            where_clause += "Active = :inactive";
            parameters.push((String::from("inactive"), mysql::Value::Int(0)));
        }

        if where_clause == "WHERE " { where_clause = String::new() }

        let param: mysql::Params = match parameters.len() {
                    0 => mysql::Params::from(()),
                    _ => mysql::Params::from(parameters),
                };

        //Execute query
        let query_result =
            self.connection.prep_exec(
                format!(
                    "SELECT * FROM keepers {}", 
                    where_clause
                ),
                param
            );

        match query_result {
            Ok(value) => {
                let mut rows: Vec<mysql::Row> = Vec::new();

                for row in value {
                    rows.push(row.unwrap());
                }

                match rows.len() {
                    0 => Err(()),
                    _ => Ok(rows)
                }
            },
            Err(_e) => Err(()) 
        }
    }
}