extern crate serde_json;


 mod router;
 use router::route;



fn main() {
       let input = r#"{
                    "route": "login",
                    "action":{
                        "user":"U",
                        "password":"P"
                    }
                  }"#;

    route(input); 
    
}

