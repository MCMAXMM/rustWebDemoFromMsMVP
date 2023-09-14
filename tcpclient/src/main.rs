use std::io::{Read,Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream=TcpStream::connect("localhost:3000").unwrap();
    stream.write("hello world".as_bytes()).unwrap();
    let mut buffer=[0;20];
    let length=stream.read(&mut buffer).unwrap();
    println!("length: {}", length);
    println!("response from server:{:?}",str::from_utf8(&buffer[..length]).unwrap());
}
