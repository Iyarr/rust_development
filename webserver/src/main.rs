use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        // 接続が確立しました
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";

    let mut contents = String::new();

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(get) {
        let mut file = File::open("index.html").unwrap();
        file.read_to_string(&mut contents).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
    } else {
        let contents = String::from("<html><head><meta charset='UTF-8'/></head><body><H1>のっとふぁうんど</H1></body></html>");
        let response = format!("HTTP/1.1 400 Not Found\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
