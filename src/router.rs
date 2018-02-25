extern crate serde_json;

use serde_json::{Value, Error};


fn main(){
    /* TODO: get input as argument */
    
    let input = r#"{
                    "route": "signup",
                    "action":{
                        "A":"A"
                    }
                  }"#;

    route(input); 
}

fn route(s : &str) -> Result<(), Error> {
    let v: Value = serde_json::from_str(s)?;

    match v["route"].to_string().as_str() {
        "\"login\"" => login(),
        "\"signup\"" => signUp(),
        _ => notFound(),
    }

    Ok(())
}

fn notFound() {
    println!("not found!");
}

fn login() {
    println!("login!");
}

fn signUp() {
    println!("sign-up");
}