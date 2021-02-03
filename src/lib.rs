extern crate clap;
use clap::{App, Arg};

mod tcpstream;
mod udpsocket;

pub fn run() {
    let matches = App::new("Text Transfer")
        .version("0.1.1")
        .author("Abner K. <abnerkaizer@protonmail.com>")
        .about("Transfers a text file through UDP or TCP. UDP is the default one.")
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("TCP")
                .about("Set TCP as the protocol of choice.Default is false.")
                .short('s')
                .long("set-tcp")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let input = matches.value_of("INPUT");
    let input = match input {
        Some(inp) => inp,
        None => "",
    };
    let tcp = matches.value_of("TCP");

    let t: bool = match tcp {
        Some(b) => b.trim().parse().expect("Boolean was expected find"),
        None => false,
    };
    if t {
        tcpstream::run(input.to_string());
    } else {
        udpsocket::run(input.to_string());
    }
}
