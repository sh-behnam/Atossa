extern crate robotparser;
extern crate url;

use url::Url;
use robotparser::RobotFileParser;

mod Crawler;
use Crawler::Agent;

fn main() {

    let url = "http://www.um.ac.ir/".to_string();
    let host = Agent::parse_url("host", &*url);
    let scheme = Agent::parse_url("scheme", &*url);
    let url_object = Url::parse(&url).unwrap();
    let relative_url = "/".to_string() + &url_object.path().unwrap().join("/");
    let base_url = scheme.to_string() + "://" + &*host;
    let robots_txt_url = base_url; //+ "/robots.txt";

    println!("url: {}", url);
    //println!("host: {}", host);
    //println!("scheme: {}", scheme);
    //println!("relative_url: {}", relative_url);
    //println!("base_url: {}", base_url);

    //Check if we are allowed to crawl via robots.txt
    let parser = RobotFileParser::new(&*robots_txt_url);
    parser.read();
    if parser.can_fetch("*", &*relative_url) {
    	Agent::crawl(&*url);
    }

}
