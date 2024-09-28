// use server::{HttpSchemeEnum, Tag, Tokenizer, Uri};

use html_messages::errors::MessageParseError;
use html_messages::request::RequestMessage;
use html_messages::response::ResponseMessage;
use html_shared::method::HTTPMethod;
use router::route::Route;
use router::router::Router;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> Result<(), MessageParseError> {
    File::create("request.log").expect("Could not create request.log");

    let mut logs = OpenOptions::new()
        .append(true)
        .open("request.log")
        .expect("Cannot write to request.log");

    let mut router = Router::default();

    router.connect_route(
        Route(HTTPMethod::GET, "/".to_string()),
        "files/test_file.html".to_string(),
    );

    let address = "127.0.0.1:8080";
    println!("Opening listener on http://{} . . .", address);
    let stream = TcpListener::bind(address).expect("");

    for s in stream.incoming() {
        let mut handle = match s {
            Ok(handle) => handle,
            Err(_) => continue,
        };
        // println!("Received a stream: {:?}", &handle);
        let mut buffer = [0; 8000];
        handle.read(&mut buffer).expect("");

        let _ = logs.write(&buffer);
        let _ = logs.write("\n".as_bytes());

        let request = RequestMessage::parse_request(&buffer)?;
        println!("Received:\n\n{:?}\n\n", request);

        let response = ResponseMessage::build_response(request, &router).expect("");
        println!("Responded with:\n========\n{}=======\n\n", response);
        handle.write_all(response.to_string().as_bytes()).expect("");
        handle.flush().expect("couldnt flush buffer");
    }
    Ok(())
}
