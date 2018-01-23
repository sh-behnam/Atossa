use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

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
    fn do_request(){
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());
        let uri = "http://httpbin.org/ip".parse().unwrap();
        let work = client.get(uri).and_then(|res| {
            println!("Response: {}", res.status());

            res.body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(From::from)
            })
        });
        core.run(work).unwrap();
    }
}