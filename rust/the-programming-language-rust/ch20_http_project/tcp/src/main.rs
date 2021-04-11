extern crate tcp;
use tcp::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let ip = "127.0.0.1";
    let port = "7878";
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).expect("TcpListener bind failed");
    let pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);
    //println!("{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
