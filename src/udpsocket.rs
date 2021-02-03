use std::fs;
use std::net::UdpSocket;
use std::process::exit;

pub fn run(input: String) {
    let content = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Couldn't read file, error: {}", e);
            exit(1)
        }
    };
    let socket = match UdpSocket::bind("localhost:30789") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Couldn't bind to socket, error: {}", e);
            exit(1);
        }
    };
    match socket.connect("localhost:6789") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Couldn't connect to socket, error: {}", e);
            exit(1);
        }
    };

    let num = match socket.send(content.as_bytes()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Couldn't send to socket, error: {}", e);
            exit(1);
        }
    };
    println!("{}", num);
}
