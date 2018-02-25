mod Database;

extern crate rusqlite;

use rusqlite::Connection;
use Database::Query;

fn main() {
    println!("Hello, world!");
    #[derive(Clone, Copy)]
    let conn = Connection::open_in_memory().unwrap();
    let query = Query::new(conn);

    let user_table =
    "CREATE TABLE user (
        user_id              INTEGER PRIMARY KEY,
        user_name            STRING UNIQUE,
        user_email           STRING UNIQUE,
        user_password        STRING
    )".to_string();


    let role_table =
    "CREATE TABLE role (
        role_id              INTEGER PRIMARY KEY,
        role_name            STRING UNIQUE
    )".to_string();

    let user_role =
    "CREATE TABLE role (
        user_id              INTEGER PRIMARY KEY,
        role_id            INTEGER PRIMARY KEY
    )".to_string();

    //query.perform(&conn, user_table);
}
