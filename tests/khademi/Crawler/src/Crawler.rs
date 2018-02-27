extern crate hyper;
extern crate select;
extern crate xhtmlchardet;
extern crate robotparser;
extern crate url;

use std::io::Read;
use Crawler::hyper::client::Client;
use Crawler::hyper::header::Connection;
use Crawler::select::document::Document;
use Crawler::select::predicate::*;

pub enum ParseOption {
    HostName,
    Port,
    Protocol,
    Username,
    Password,
    Path,
    Fragment,
    Query
}

#[derive(Debug)]
pub struct Agent {

}

impl Agent {
    pub fn new() -> Agent {
        Agent {

        }
    }
}

impl Agent {

    pub fn get_parse_option(arg: &str) -> ParseOption {
        let option_prefix_chars: &[char] = &['-'];
        let trimmed_arg = arg.trim_matches(option_prefix_chars);

        match trimmed_arg {
            "host"     => ParseOption::HostName,
            "port"     => ParseOption::Port,
            "protocol" => ParseOption::Protocol,
            "password" => ParseOption::Password,
            "username" => ParseOption::Username,
            "path"     => ParseOption::Path,
            "fragment" => ParseOption::Fragment,
            "query"    => ParseOption::Query,
            "scheme"   => ParseOption::Protocol,
            _          => panic!("Invalid parsing option")
        }
    }

    pub fn parse_url(option: &str, url: &str) -> String {

        let result = url::Url::parse(url);

        match result {
            Err(e) => {
                return e.to_string();
            },
            _ => {}
        };

        let parsed = result.unwrap();

        let value = match Agent::get_parse_option(option) {
            ParseOption::HostName => Agent::parse_component(parsed.domain(), "hostname"),
            ParseOption::Port     => Agent::parse_component(parsed.port(), "port"),
            ParseOption::Protocol => Agent::parse_component(Some(parsed.scheme), "scheme"),
            ParseOption::Username => Agent::parse_component(parsed.username(), "username"),
            ParseOption::Password => Agent::parse_component(parsed.password(), "password"),
            ParseOption::Path     => Agent::parse_component(parsed.serialize_path(), "path"),
            ParseOption::Fragment => Agent::parse_component(parsed.fragment, "fragment"),
            ParseOption::Query    => Agent::parse_component(parsed.query, "query")
        };

        match value {
            Ok(v) => {
                let cval = v.to_string();
                return cval;
            }
            Err(e) => {
                return e.to_string();
            }
        };

    }

    pub fn parse_component<T: ToString>(option: Option<T>, description: &str) -> Result<String, String> {
        return match option {
            Some(x) => Ok(x.to_string()),
            None    => Err(format!("No {} found", description))
        }
    }

    pub fn crawl(url: &str) {

        //Opens up a new HTTP client
        let client = Client::new();

        //Creates outgoing request
        let mut res = client.get(&*url)
            .header(Connection::close())
            .send().unwrap();

        //Reads the response
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        //println!("Response: {}", res.status);
        //println!("Headers:\n{}", res.headers);
        //println!("Body:\n{}", body);

        let document = Document::from_str(&*body);

        for node in document.find(Attr("id", "responsive-menu")).iter() {
            println!("({:?})\n\n", node);
        }
    }
}
