use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        println!("Connection from: {}", peer_addr); // クライアントのIPを表示
        handle_connection(stream);
        // 接続が確立しました
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    stream.read(&mut buffer).unwrap();
    let (first_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    } 
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    }
    else {
        ("HTTP/1.1 400 Not Found\r\n\r\n","404.html")
    };

    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}", first_line,contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
