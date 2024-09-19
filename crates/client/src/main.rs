use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let string = "https://telemakos.io/test?test_query";
    println!("Sending {} . . .", string);
    stream.write_all(string.as_bytes())?;
    // stream.read(&mut [0; 128])?;
    Ok(())
}
