use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

#[derive(Debug)]
pub struct Request {
    header: Header,
//    body: String
} 
#[derive(Debug)]
pub struct Header{
    method: Method,
    uri: String,
//    version: String
}
#[derive(Debug)]
pub enum Protocol {
    HTTP,
//    HTTPS
}
#[derive(Debug)]
pub enum Method {
    GET,
//    POST
}

#[derive(Debug)]
pub struct HTTP {
    address: String,
    port: u16,
    request: Request,
    protocol: Protocol,
}

impl HTTP {
    pub fn new(address: String, port: u16, protocol: Protocol, method: Method, uri: String) -> HTTP {
        HTTP {
            address: address,
            port: port,
            protocol: protocol,
            request: Request {
                header: Header {
                    method: method,
                    uri: uri,
                },
            },
        }
    }
    pub fn do_request(&self){
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

/*
#[derive(Default)]
pub struct HTTP<'a> {
    Request: Request<'a>,
    Response: Response<'a>,
}
#[derive(Default)]
struct Request {
    Header: String,
    Body: String,
}
#[derive(Default)]
struct Response {
    Header: String,
    Body: String,
}

impl<'a> HTTP<'a> {
    fn get_request() -> String {
        String::from("")
    }
    fn get_response() -> String {
        String::from("")
    }
    pub fn set_request(){
        
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
*/