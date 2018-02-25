extern crate rusqlite;

/*
2- Execute queries to create tables
4- Test tables using a login scenario
*/

use rusqlite::Connection;

/* Table Structure */
#[derive(Debug)]
pub struct User {
    user_id: i32, //PRIMARY
    user_name: String, //UNIQUE
    user_email: String, //UNIQUE
    user_password: String,
}

#[derive(Debug)]
pub struct Role {
    role_id: i32, //PRIMARY
    role_name: String, //UNIQUE
}

#[derive(Debug)]
pub struct User_Role {
    user_id: i32,
    role_id: i32,
}

    /* Query Class Decleration */
    #[derive(Debug)]
    pub struct Query {
        conn: Connection,
    }

    impl Query {
        //Constructs "Query" class
        pub fn new(conn: Connection) -> Query {
            Query {
                conn: conn,
            }
        }
    }

    impl Query {
        pub fn perform(&self, conn: &Connection, command: String) {
            //Executes the "query"
            conn.execute(&command, &[]).unwrap();
        }
        /*
        let conn = Connection::open_in_memory().unwrap();
        let query = "CREATE TABLE person (
                          id              INTEGER PRIMARY KEY,
                          name            TEXT NOT NULL,
                          data            BLOB
                          )";
        OR
        let query = "INSERT INTO person (name, data)
                          VALUES (?1, ?2)";
        conn.execute(query, &[&me.name, &me.data]).unwrap();
        conn.execute(query, &[]).unwrap();
        */

        pub fn login(&self, conn: Connection, user: User) -> bool {
            //Writes a "SELECT" query
            let query = "SELECT user_id, user_name, user_email, user_password FROM user";

            //Outputs the query into "stmt"
            let mut stmt = conn.prepare(query).unwrap();

            //Creates an iterator for the query
            let user_iter = stmt.query_map(&[], |row| {
                User {
                    user_id: row.get(0),
                    user_name: row.get(1),
                    user_email: row.get(2),
                    user_password: row.get(3),
                }
            }).unwrap();

            //Finds "user" in "user_iter"
            for element in user_iter {
                let candidate = element.ok().unwrap();
                if candidate.user_name == user.user_name {
                    if candidate.user_password == user.user_password {
                        println!("Login successful!");
                        return true;
                    }
                }
            }
            return false;
        }
        /*
        let conn = Connection::open_in_memory().unwrap();
        */
    }
