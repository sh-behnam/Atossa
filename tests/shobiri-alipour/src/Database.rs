extern crate rusqlite;

use self::rusqlite::Connection;

/* Table Structure */
#[derive(Debug)]
pub struct User {
    user_id: i32, //PRIMARY
    user_name: String, //UNIQUE
    user_email: String, //UNIQUE
    user_password: String,
}

impl User {
    //Constructs "Query" class
    pub fn new(user_id: i32,
    user_name: String,
    user_email: String,
    user_password: String,
    ) -> User {
        User {
            user_id: user_id,
            user_name: user_name,
            user_email: user_email,
            user_password: user_password,
        }
    }
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

}

impl Query {

    //Constructs "Query" class
    pub fn new() -> Query {
        Query {

        }
    }
}

impl Query {

    pub fn perform(&self, conn: Connection, command: String) {

        //Executes the "query"
        conn.execute(&command, &[]).unwrap();
    }

    pub fn insert_user(&self, conn: Connection, user: User) {

        //Writes a "INSERT" query
        let command = "INSERT INTO user (user_id, user_name, user_email, user_password)
                  VALUES (?1, ?2, ?3, ?4)";

        //Executes the "query"
        conn.execute(command, &[&user.user_id, &user.user_name, &user.user_email, &user.user_password]).unwrap();
    }

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
}
