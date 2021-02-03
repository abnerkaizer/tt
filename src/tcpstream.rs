use num_cpus;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

pub fn run(input: String) {
    let pool = ThreadPool::new(num_cpus::get());
    let inp = input.clone();
    let mut content = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Couldn't read file, error:{}", e);
            std::process::exit(1)
        }
    };
    let listener = TcpListener::bind("localhost:6789").expect("Failed to bind.");
    let stream = listener.incoming().next().unwrap().unwrap();
    let cont = content.clone();
    pool.execute(|| handle_connection(stream, inp, cont));
    let mut stream = TcpStream::connect("localhost:6789").expect("Failed to connect.");
    unsafe {
        match stream.read(content.as_bytes_mut()) {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Couldn't read stream, error:{}", e);
                std::process::exit(1)
            }
        };
    }
}
fn handle_connection(mut stream: TcpStream, _input: String, content: String) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't read buffer, error:{}", e);
            std::process::exit(1)
        }
    };
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    match stream.write(response.as_bytes()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't write into stream, error:{}", e);
            std::process::exit(1)
        }
    };
    match stream.flush() {
        Ok(flush) => flush,
        Err(e) => {
            eprintln!("Couldn't flush stream, error:{}", e);
            std::process::exit(1)
        }
    };
    let response = content;
    match stream.write(response.as_bytes()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't write into stream, error:{}", e);
            std::process::exit(1)
        }
    };
    match stream.flush() {
        Ok(flush) => flush,
        Err(e) => {
            eprintln!("Couldn't flush stream, error:{}", e);
            std::process::exit(1)
        }
    };
}
