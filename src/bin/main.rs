use std::thread;
use std::fs::File;
use std::time::Duration;
use std::thread::Thread;
use std::io::{Read, Write};
use web_server::ThreadPool;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let thread_pool = ThreadPool::new(4);

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let content = String::from_utf8_lossy(&buffer[..]);
    println!("Request {}", content);

    let request_line = b"GET / HTTP/1.1\r\n";
    let sleep_line = b"GET /sleep HTTP/1.1\r\n";

    let (response_line, html_file) = if buffer.starts_with(request_line) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep_line) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut html_content = String::new();
    let mut file = File::open(html_file).unwrap();
    file.read_to_string(&mut html_content).unwrap();

    let response = format!("{}{}", response_line, html_content);
    stream.write(&response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
