// use server::{HttpSchemeEnum, Tag, Tokenizer, Uri};

use html_messages::errors::MessageParseError;
use html_messages::request::RequestMessage;
use html_messages::response::ResponseMessage;
use html_shared::method::HTTPMethod;
use router::route::Route;
use router::router::Router;
use std::path::PathBuf;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> Result<(), MessageParseError> {
    let mut router = Router::default();

    router.connect(
        Route(HTTPMethod::GET, "/".to_string()),
        PathBuf::from("files/test_file.html"),
    );

    // router.connect(
    //     Route(HTTPMethod::GET, "/final".to_string()),
    //     PathBuf::from("files/citc1300-final-master/index.html"),
    // );

    let address = "127.0.0.1:8080";
    println!("Opening listener on http://{} . . .", address);
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

        let response = ResponseMessage::build_response(request, &router).expect("");
        println!("Responded with:\n========\n{}=======\n\n", response);
        handle.write_all(response.to_string().as_bytes()).expect("");
        handle.flush().expect("couldnt flush buffer");
    }
    Ok(())
}
