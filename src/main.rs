mod thread_pool;

use crate::thread_pool::ThreadPool;
use std::{fs};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute( || {
           handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer);
    let request_args: Vec<&str> = request_str.split_whitespace().collect();
    let path = format!("frontend{}", request_args.get(1).unwrap());

    let (status_line, filepath) =
        if Path::new(path.as_str()).exists() && Path::new(path.as_str()).is_file() {
            ("HTTP/1.1 200 OK", path.as_str())
        } else if Path::new(path.as_str()).is_dir() {
            ("HTTP/1.1 200 OK", "frontend/home.html")
        } else {
            ("HTTP/1.1 200 OK", "frontend/err.html")
        };

    println!("{}", filepath);
    let contents = fs::read_to_string(filepath).unwrap();

    let response =
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}