// use server::{HttpSchemeEnum, Tag, Tokenizer, Uri};

use request::{MessageParseError, RequestMessage};
use response::ResponseMessage;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub mod request;
pub mod response;

fn main() -> Result<(), MessageParseError> {
    //     let buffer = r##"GET /help/img/idea/2024.2/ps_http_client_unresolved_var.png HTTP/1.1
    // Host: resources.jetbrains.com
    // User-Agent: Mozilla/5.0 (Windows NT 10.0; rv:130.0) Gecko/20100101 Firefox/130.0
    // Accept: image/avif,image/webp,image/png,image/svg+xml,image/*;q=0.8,*/*;q=0.5
    // Accept-Language: en-US,en;q=0.5
    // Accept-Encoding: gzip, deflate, br, zstd
    // Referer: https://www.jetbrains.com/

    // <test body>"}"##;

    //     let put_buffer = r##"PUT /test_file.html HTTP/1.1

    // <p>NewFile</p>"##;

    //     let get_buffer = r##"GET /test_file.html HTTP/1.1
    // "##;

    //     let put_request = RequestMessage::parse_request(put_buffer.as_bytes())?;
    //     println!("Parsed PUT\n\n{}\n\nto\n\n{}\n", put_buffer, put_request);
    //     let put_response = ResponseMessage::build_response(put_request).expect("");
    //     println!("Response PUT:\n{}\n\n", put_response);

    //     let get_request = RequestMessage::parse_request(get_buffer.as_bytes())?;
    //     println!("Parsed GET\n\n{}\n\nto\n\n{}\n", get_buffer, get_request);
    //     let get_response = ResponseMessage::build_response(get_request).expect("");
    //     println!("Response GET:\n{}\n\n", get_response);

    let address = "127.0.0.1:8080";
    println!("Opening listener on {} . . .", address);
    let stream = TcpListener::bind(address).expect("");

    for s in stream.incoming() {
        let mut handle = match s {
            Ok(handle) => handle,
            Err(_) => continue,
        };
        // println!("Received a stream: {:?}", &handle);
        let mut buffer = [0; 512];
        handle.read(&mut buffer).expect("");

        let request = RequestMessage::parse_request(&buffer)?;
        println!("Received:\n\n{:?}\n\n", request);

        let response = ResponseMessage::build_response(request).expect("");
        println!("Responded with:========\n\n{}=======", response);
        handle.write_all(response.to_string().as_bytes()).expect("");
        handle.flush().expect("couldnt flush buffer");
    }
    Ok(())
}
