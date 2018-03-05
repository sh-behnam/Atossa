use hyper;
use tokio_core;
use std::io::{self, Write};
use futures::{Future, Stream};

pub use hyper::Method;
use cookie::{Cookie, CookieJar};

#[derive(Debug)]
pub struct Request {
    header: RequestHeader,
    body: String,
}
#[derive(Default)]
struct Response {
    header: ResponseHeader,
    body: String,
}
#[derive(Debug)]
pub struct RequestHeader {
    method: Method,
    cookie: String,
    uri: String,
    //    version: String
}
#[derive(Default)]
pub struct ResponseHeader {
//    method: Method,
//    uri: String,
//    version: String
}
#[derive(Debug)]
pub enum Protocol {
    HTTP,
    //    HTTPS
}

#[derive(Debug)]
pub struct HTTP {
    address: String,
    port: u16,
    request: Request,
    protocol: Protocol,
}

impl HTTP {
    pub fn new(
        address: String,
        port: u16,
        protocol: Protocol,
        method: Method,
        uri: String,
    ) -> HTTP {
        HTTP {
            address: address,
            port: port,
            protocol: protocol,
            request: Request {
                header: RequestHeader {
                    method: method,
                    cookie: String::from(""),
                    uri: uri,
                },
                body: String::from(""),
                //                body: "".into_string(),
            },
        }
    }
    pub fn set_body(&mut self, data: String) {
        self.request.body = data;
    }

    pub fn set_cookie(&mut self, data: String) {
        self.request.header.cookie = data; // temp
    }
    pub fn do_request(&self) {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let client = hyper::Client::new(&core.handle());
        let uri = format!(
            "{}://{}:{}/{}",
            match self.protocol {
                Protocol::HTTP => "http",
                //                    Protocol::HTTPS => "https",
            },
            self.address,
            self.port,
            self.request.header.uri
        ).parse()
            .unwrap();
        let mut req = hyper::Request::new(self.request.header.method.clone(), uri);
        match self.request.header.method {
            Method::Get => {}
            Method::Post => {
                req.set_body(self.request.body.clone());
            }
            Method::Options => {}
            Method::Put => {
                req.set_body(self.request.body.clone());
            }
            Method::Delete => {}
            Method::Head => {}
            Method::Trace => {}
            Method::Connect => {}
            Method::Patch => {}
            _ => {}
        };
        req.headers_mut().set(hyper::header::ContentLength(2u64));
        let work = client.request(req).and_then(|res| {
            println!("Response: {}", res.status());
            res.body()
                .for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
        });
        core.run(work).unwrap();
    }
}
