extern crate rusqlite;

use rusqlite::Connection;

/* Table Structure */
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

/* Query Class Decleration */
#[derive(Debug)]
pub struct Query {
    conn: Connection,
}

/* Query Class Implementation */
//let conn = Connection::open_in_memory().unwrap(); //Open a connection
impl Query {
    //Constructor
    pub fn new(
        conn: Connection,
    ) -> Query {
        Query {
            conn: conn,
        }
    }

    pub fn execute(&self, conn: Connection) {
        //Write a datbase query
        conn.execute("CREATE TABLE person (
                      id              INTEGER PRIMARY KEY,
                      name            TEXT NOT NULL,
                      data            BLOB
                      )", &[]).unwrap();

        //Instance of "Person"
        let me = Person {
            id: 0,
            name: "Steven".to_string(),
            data: None
        };

        //Write a database query
        conn.execute("INSERT INTO person (name, data)
                      VALUES (?1, ?2)",
                     &[&me.name, &me.data]).unwrap();
    }

    pub fn report(&self, conn: Connection) {
        //Write a database query
        let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();

        //Create an iterator for the query
        let person_iter = stmt.query_map(&[], |row| {
            Person {
                id: row.get(0),
                name: row.get(1),
                data: row.get(2)
            }
        }).unwrap();

        //Find specific person in iterator
        for person in person_iter {
            println!("Found person {:?}", person.unwrap());
        }
    }
}
