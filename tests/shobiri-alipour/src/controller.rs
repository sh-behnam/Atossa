extern crate rusqlite;
use rusqlite::Connection;

mod Database;
use Database::Query;
use Database::User;

pub fn controller_login_check(user : String,pass : String) 
{
    println!("the username is {} and ",user);
    println!("the password is {} and ",pass);
       
    let path = "./DEV_DB_TEST";
    let conn = Connection::open(path).unwrap();
    let query = Query::new();

    let user = User::new(
        12,
        user,
        "admin@gmail.com".to_string(),
        pass
    );

    let ans = query.login(conn,user);
    
}