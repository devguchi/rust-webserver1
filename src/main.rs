use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let mut contents = String::new();
    let mut status_code = 200;
    let mut status_msg = "OK";
    if buffer.starts_with(get) {
        let mut file = File::open("a.html").unwrap();
        file.read_to_string(&mut contents).unwrap();
    } else {
        status_code = 400;
        status_msg = "NOT FOUND";
        contents = String::from("NOT FOUND");
    }
    let response = format!(
        "HTTP/1.1 {} {}\r\n\r\n{}",
        status_code, status_msg, contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
