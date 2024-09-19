use std::net::TcpListener;

use server::{HttpRequest, Uri};

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    println!("Opening listener on {} . . .", address);
    let stream = TcpListener::bind(address)?;

    for s in stream.incoming() {
        println!("Received a stream: {:?}", &s);
        let mut buffer = [0; 8000];
        // let mut converted_buffer = String::new();
        s?.peek(&mut buffer)?;
        let converted_buffer = Uri::parse_buffer(&buffer);
        println!("Received:\n\t{:?}", converted_buffer);
    }
    // stream.write(&[1])?;
    Ok(())
}
