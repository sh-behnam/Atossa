mod http;
mod Crawler;

pub enum Vulnerabillity {
    LOW,
    MEDIUM,
    HIGH
}

pub enum Option {
    DIRECTORY_SEARCH,
    SQL_INJECTION,
}

pub struct Output {
    input: String,
    output: String,
    http: http
}

impl Output {
    pub fn new(
        input: String,
        output: u16,
        http: http,
    ) -> Output {
        Output {
            input: input,
            output: output,
            http: http,
        }
    }
}

pub struct Fuzzer {
    dir_list: Vec<String>,
    http_list: Vec<http>,
}

impl Fuzzer {
    pub fn new(
        dir_list: Vec<String>
    ) -> Fuzzer {
        Fuzzer {
            dir_list: dir_list,
        }
    }
    pub fn execute(&self, option: Option, http: http) {
        match option {
            DIRECTORY_SEARCH => self.dir_search(http),
        }
    }
}

impl Fuzzer {
    pub fn dir_search(http: http) {
        http_list: Vec<http>;
        for dir in dir_list.iter() {
            _http = http.clone();
            _http.uri = format!("{}{}", http.uri, dir);
            _http.do_request();
            http_list.push(_http);
        }
    }
}
