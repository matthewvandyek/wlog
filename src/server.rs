use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*; // import contents "INSIDE" the prelude module

const HTTP_OK: &str = "HTTP/1.1 200 OK\r\n";
const HTTP_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n";

const URL: &str = "0.0.0.0:7878";

pub fn listen() -> TcpListener {
    let listener = TcpListener::bind(URL).unwrap();
    println!("Listening on {}...", URL);
    return listener;
}

pub fn connect(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        (HTTP_OK, "assets/index.html")
    } else {
        (HTTP_NOT_FOUND, "assets/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}