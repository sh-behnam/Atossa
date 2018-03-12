extern crate rusqlite;
extern crate serde_json;

use self::serde_json::{Value, Error};

use Database::Query;
use Database::User;
use self::rusqlite::Connection;

pub fn login_check(JSON : &Value) -> bool
{
    let  mut _username = JSON["action"]["user"].to_string() ; //pars JSON for username
    _username = _username[1.._username.len()-1].to_string();// delet " " form username to compare

    let  mut _password = JSON["action"]["password"].to_string(); //pars JSON for password
    _password = _password[1.._password.len()-1].to_string();// delet " " form password to compare

    println!("the username is {} and ",_username);
    println!("the password is {} and ",_password);

    
    let path = "./DEV_DB_TEST";
    let conn = Connection::open(path).unwrap();
    let query = Query::new();
    
    //create new user with user and pass (but fake id and mail)
    let user = User::new(
        12, // test id
        _username,
        "admin@gmail.com".to_string(), // test email
        _password
    );

    let ans = query.login(conn,user);
    if ans {
        println!("welcome");
        true
    }
    else {
        println!("username or password is not correct");
        false
    }
}