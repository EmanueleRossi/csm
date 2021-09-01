use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;


#[test]
fn test_reboot() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");
            stream.write(b"reboot").unwrap();
            println!("Sent reboot, awaiting reply...");
            let mut data = [0 as u8; 6];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Unexpected reply: {}", text);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
