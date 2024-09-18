use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let stream = TcpListener::bind("127.0.0.1:8080")?;

    for s in stream.incoming() {
        let mut buffer = [0; 10];
        s?.peek(&mut buffer)?;
        println!("{:?}", buffer);
    }
    // stream.write(&[1])?;
    Ok(())
}
