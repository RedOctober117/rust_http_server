// use server::{HttpSchemeEnum, Tag, Tokenizer, Uri};

use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpListener,
    stream::StreamExt,
    sync::RwLock,
    task,
};
use html_messages::errors::MessageParseError;
use html_messages::request::RequestMessage;
use html_messages::response::ResponseMessage;
use html_shared::method::HTTPMethod;
use router::route::Route;
use router::router::Router;
use std::{ fs::File, sync::Arc};

#[async_std::main]
async fn main() -> Result<(), MessageParseError> {
    File::create("request.log").expect("Could not create request.log");

    // let mut logs = OpenOptions::new()
    //     .append(true)
    //     .open("request.log")
    //     .expect("Cannot write to request.log");

    let router = Arc::new(RwLock::new(Router::default()));

    router.write_blocking().connect_recursive_routes(
        Route(HTTPMethod::GET, "/".to_string()),
        "files/test_file.html".to_string(),
    );

    let address = "127.0.0.1:8080";
    println!("Opening listener on http://{} . . .", address);
    let stream = TcpListener::bind(address).await.expect("");
    let mut incoming = stream.incoming();

    while let Some(s) = incoming.next().await {
        let router_ptr = Arc::clone(&router);
        task::spawn(async {
            let mut handle = match s {
                Ok(handle) => handle.clone(),
                Err(_) => todo!(),
            };

            let mut buffer = [0; 1000];
            handle.read(&mut buffer).await.expect("");

            // let _ = logs.write(&buffer);
            // let _ = logs.write("\n".as_bytes());

            let request = RequestMessage::parse_request(&buffer).unwrap();
            println!("Received:\n\n{:?}\n\n", request);

            let response = ResponseMessage::build_response(request, router_ptr)
                .await
                .expect("");
            println!("Responded with:\n========\n{}=======\n\n", response);
            handle
                .write_all(response.to_string().as_bytes())
                .await
                .expect("");
            handle.flush().await.expect("couldnt flush buffer");
        })
        .await
    }
    Ok(())
}
