use tiny_http::{Request, Response, Server};

pub fn start_http_server() {
    std::thread::spawn(move || {
        let server = Server::http("127.0.0.1:8000").unwrap();
        for request in server.incoming_requests() {
            handle_request(request);
        }
    });
}

fn handle_request(request: Request) {
    let url = request.url();

    // Parse the URL and take its query parameters
    if let Some(query_start) = url.find('?') {
        let path = &url[..query_start];
        let query = &url[query_start+1..];

        // Split the query string into key-value pairs
        let params: Vec<&str> = query.split('&').collect();

        if path == "/reload" {
            for param in params {
                let pair: Vec<&str> = param.split('=').collect();
                if pair[0] == "lib" {
                    let lib_name = pair[1];
                    println!("Reloading library: {}", lib_name);
                    // Unload and reload the library here
                }
            }
        }
    }

    let response = Response::from_string("Received");
    request.respond(response).unwrap();
}
