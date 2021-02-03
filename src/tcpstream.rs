use num_cpus;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use threadpool::ThreadPool;

pub fn run(input: String) {
    let pool = ThreadPool::new(num_cpus::get());
    let mut content = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Couldn't read file, error: {}", e);
            exit(1)
        }
    };

    let listener = match TcpListener::bind("localhost:6789") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Couldn't bind to socket, error: {}", e);
            exit(1)
        }
    };

    for stream in listener.incoming() {
        let cont = content.clone();
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Couldn't open stream, error: {}", e);
                exit(1)
            }
        };
        pool.execute(move || handle_connection(stream, cont));
    }

    let mut stream = match TcpStream::connect("localhost:6789") {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Couldn't connect, error: {}", e);
            exit(1)
        }
    };
    unsafe {
        match stream.read(content.as_bytes_mut()) {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Couldn't read stream, error: {}", e);
                exit(1)
            }
        };
    }
}
fn handle_connection(mut stream: TcpStream, content: String) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't read buffer, error: {}", e);
            exit(1)
        }
    };
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    match stream.write(response.as_bytes()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't write into stream, error: {}", e);
            exit(1)
        }
    };
    match stream.flush() {
        Ok(flush) => flush,
        Err(e) => {
            eprintln!("Couldn't flush stream, error: {}", e);
            exit(1)
        }
    };
    let response = content;
    match stream.write(response.as_bytes()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't write into stream, error: {}", e);
            exit(1)
        }
    };
    match stream.flush() {
        Ok(flush) => flush,
        Err(e) => {
            eprintln!("Couldn't flush stream, error: {}", e);
            exit(1)
        }
    };
}
