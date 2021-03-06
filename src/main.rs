extern crate system_shutdown;

use system_shutdown::force_reboot;
use system_shutdown::force_shutdown;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string();
    if request.eq("shutdown") {
        println!("shutdown Request!: {}", request);
        match force_shutdown() {
            Ok(_) => println!("Bye!"),
            Err(error) => eprintln!("Failed to shutdown: {}", error)
        }
    } else if request.eq("reboot") {
        println!("reboot Request!: {}", request);
        match force_reboot() {
            Ok(_) => println!("Bye!"),
            Err(error) => eprintln!("Failed to reboot: {}", error)
        }
    } else {
        println!("Request: {:#?}", request);
    }
}
