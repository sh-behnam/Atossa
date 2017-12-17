struct HTTP<'a> {
    Request: Request<'a>,
    Response: Response<'a>,
}
struct Request<'a> {
    Header: &'a str,
    Body: &'a str,
}
struct Response<'a> {
    Header: &'a str,
    Body: &'a str,
}

impl<'a> HTTP<'a> {
    fn get_request() -> String {
        String::from("")
    }
    fn get_response() -> String {
        String::from("")
    }
    fn set_request(s: &str){
        
    }
}