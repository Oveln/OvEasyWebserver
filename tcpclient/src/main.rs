use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut steram = TcpStream::connect("localhost:3000").unwrap();
    steram.write("Hello TCPServer".as_bytes()).unwrap();

    let mut buffer = [0;5];
    steram.read(&mut buffer).unwrap();
    
    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
