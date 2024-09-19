use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let stream = TcpListener::bind("127.0.0.1:8080")?;

    for s in stream.incoming() {
        let mut buffer = [0; 8000];
        let mut converted_buffer = String::new();
        s?.peek(&mut buffer)?;
        for item in buffer {
            if item as char != '\0' {
                converted_buffer.push(item as char);
            }
        }
        println!("{:?}", converted_buffer);
    }
    // stream.write(&[1])?;
    Ok(())
}
