mod webservice;
mod crawler;

struct Request<'a> {
    Header: &'a str,
    Body: &'a str,
}
struct Response<'a> {
    Header: &'a str,
    Body: &'a str,
}
struct HTTP<'a> {
    Request: Request<'a>,
    Response: Response<'a>,
}

fn crawl(HTTP: HTTP,depth: u8) -> HTTP {
    HTTP // Mina ShahsavanPour
}

fn main() {
    println!("Hello, Scanner!");
}
