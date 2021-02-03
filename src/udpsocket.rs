use std::fs;
use std::net::UdpSocket;


pub fn run(input: String) {
    let content = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Couldn't read file, error: {}", e);
            std::process::exit(1)
        }
    };
    let socket = match UdpSocket::bind("localhost:6789") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Couldn't bind to socket, error: {}", e);
            std::process::exit(1);
        }
    };
    match socket.connect("localhost:6789") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Couldn't connect to socket, error: {}", e);
            std::process::exit(1);
        }
    };

    match socket.send(content.as_bytes()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't send to socket, error: {}", e);
            std::process::exit(1);
        }
    };
}