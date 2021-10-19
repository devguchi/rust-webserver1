use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Webserver1");
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    });
    let handle2 = thread::spawn(|| {
        println!("Webserver2");
        let listener2 = TcpListener::bind("127.0.0.1:7879").unwrap();
        for stream2 in listener2.incoming() {
            let stream2 = stream2.unwrap();
            handle_connection2(stream2);
        }
    });
    handle.join().unwrap();
    handle2.join().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    read_request(&stream);
    let contents = create_contents("a.html");
    let header = "Set-Cookie: SESID=12345; path=/";
    let response = format!("HTTP/1.1 200 OK\r\n{}\r\n\r\n{}", header, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection2(mut stream: TcpStream) {
    read_request(&stream);
    let contents = create_contents("b.html");
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn create_contents(file: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(file).unwrap();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn read_request(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
}