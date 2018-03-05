
mod http;

#[derive(Debug)]
pub struct Resource {
    http: http,
    depth: i32,
}

#[derive(Debug)]
pub struct Agent {
    depth: i32;
    resources_list: Vec<resources>;
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            depth = 0;
        }
    }
}

impl Agent {
    pub fn crawl(&self, http: http) {

        let _resources: Vec<resource>;

        if(http.is_empty()) {
                http.response = http.do_request();
        }
        else {
            let body = http.response;
        }

        let document = Document::from_str(&body);
        for node in document.find(Name("a")).iter() {
            _http = http.clone();
            if(true) { //New address
                _http.address = http.address;
            }
            if(true) { //New port
                _http.port = http.port;
            }
            if(true) { //New protocol
                _http.protocol = http.protocol;
            }
            if(true) { //New method
                _http.method = http.method;
            }
            if(true) { //New uri
                _http.uri = http.uri;
            }

            _check = true;
            for resources in resources_list.iter() {
                for resource in resources.iter() {
                    if resource.http.compare(_http) {
                        _check = false;
                    }
                }
            }
            if _check {
                _resource: Resource(_http, self.depth);
                _resources.push(_resource);
            }

            if let Some(href) = node.attr("href") {
                println!("{}", href);
            }
        }
        self.resources_list.push(_resources);
        self.depth++;
    }

    pub fn crawl(&self, depth: i32)) {

        while self.depth < depth {

            self.depth++;
        }
    }
}
