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
    let request = String::from_utf8_lossy(&buffer);
    println!("{}", request);

    let mut contents = String::new();
    let mut file = File::open("a.html").unwrap();
    file.read_to_string(&mut contents).unwrap();
    let header = "Set-Cookie: SESID=12345; path=/";
    let response = format!("HTTP/1.1 200 OK\r\n{}\r\n\r\n{}", header, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
