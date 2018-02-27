extern crate rusqlite;
use rusqlite::Connection;

mod Database;
use Database::Query;
use Database::User;

fn main() {
    let path = "./DEV_DB_TEST";
    let conn = Connection::open(path).unwrap();
    let query = Query::new();

    /*
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
    */

    /*
    let user = User::new(
        12,
        "admin".to_string(),
        "admin@gmail.com".to_string(),
        "admin".to_string()
    );
    */
}
