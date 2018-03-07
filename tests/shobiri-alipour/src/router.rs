extern crate serde_json;

mod controller;
use controller::login_check;

use serde_json::{Value, Error};

pub fn route(s : &str) -> Result<(), Error> {
    let json: Value = serde_json::from_str(s)?;

    //check the route value in JSON structure
    match json["route"].to_string().as_str() 
    {
        "\"login\"" => {
            controller_login_check(&json);
        },
        "\"signup\"" => sign_up(),
        _ => not_found(),
    }

    Ok(())
}

//this function is just for test
fn not_found() {
    println!("not found!");
}

//this function is just for test

fn sign_up() {
    println!("sign-up");
}




/*
// a test argument for the router in JSON structure 
 let input = r#"{
                    "route": "signin",
                    "action":{
                        "user":"A",
                        "password":"A"
                    }
                  }"#;

// how to call router function

    route(input); 
*/
    
