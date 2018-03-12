
mod router;
mod controller;
mod Database;

use router::route;



fn main() {
       let input = r#"{
                    "route": "login",
                    "action":{
                        "user":"admin",
                        "password":"admin"
                    }
                  }"#;

    route(input); 
    
}

