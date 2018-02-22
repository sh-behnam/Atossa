extern crate hyper;
extern crate cookie;
extern crate futures;
extern crate tokio_core;

mod http;

fn main() {
    //    let a = http::HTTP::default();
    //    a.set_request();

    use http::HTTP;
    //use http::Protocol;

    let mut _request = HTTP::new(
        "0x00.ir".to_string(),
        80,
        http::Protocol::HTTP,
        http::Method::Get,
        "/".to_string(),
    );
    _request.set_body("Ok".to_string());
    _request.do_request();
    println!("test");
}
