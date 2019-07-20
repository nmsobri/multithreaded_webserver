use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let content = String::from_utf8_lossy(&buffer[..]);
    println!("Request {}", content);

    let mut html_content = String::new();
    let mut file = File::open("index.html").unwrap();
    file.read_to_string(&mut html_content).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", html_content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
