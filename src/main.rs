extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod http;

fn main() {
    //    let a = http::HTTP::default();
    //    a.set_request();

    use http::HTTP;
    //use http::Protocol;

    let _request = HTTP::new(
        "0x00.ir".to_string(),
        80,
        http::Protocol::HTTP,
        http::Method::Get,
        "/".to_string(),
    );

    _request.do_request();
    println!("test");
}
