extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client, Request, Method};
use hyper::header::{Headers, ContentLength};
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://127.0.0.1/".parse().unwrap();
//    let mut headers = Headers::new();
//    headers.set(ContentLength(2u64));
    let mut req = Request::new(Method::Post, uri);
    req.set_body("ss");
    req.headers_mut().set(ContentLength(2u64));
    let work = client.request(req).and_then(|res| {
        println!("Response: {}", res.status());
        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });
    core.run(work).unwrap();
}