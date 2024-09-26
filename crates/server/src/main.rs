// use server::{HttpSchemeEnum, Tag, Tokenizer, Uri};

use request::{MessageParseError, RequestMessage};
use response::ResponseMessage;

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

    let buffer = r##"PUT /test_file.html HTTP/1.1

<p>NewFile</p>"##;

    let request = RequestMessage::parse_request(buffer.as_bytes())?;
    println!("Parsed\n\n{}\n\nto\n\n{}\n", buffer, request);
    let response = ResponseMessage::build_response(request);
    println!("Response:\n{:?}", response);
    Ok(())
    // let split: Vec<_> = request.split(" \x0A").collect();

    // println!("{:?}", split.len());
    // println!("{:?}", split);
    // let test_uri = String::from("https://telemakos.io/test?test_query=#fragment-here");
    // let mut tokenizer = Tokenizer::new(test_uri.clone());

    // println!("Tokenizing . . . \n{}", tokenizer);

    // println!("Parsing {} . . .", test_uri);

    // println!("{}", Uri::parse_tokens(&mut tokenizer).ok().unwrap());

    // loop {``
    //     let token = tokenizer.next();
    //     if token.tag() == Tag::EndOfURI {
    //         break;
    //     }
    //     println!("{:?}", token.tag());
    // }
    // let address = "127.0.0.1:8080";
    // println!("Opening listener on {} . . .", address);
    // let stream = TcpListener::bind(address)?;

    // for s in stream.incoming() {
    //     println!("Received a stream: {:?}", &s);
    //     let mut buffer = [0; 8000];
    //     s?.peek(&mut buffer)?;
    //     let converted_buffer = Uri::parse_buffer(&buffer);
    //     println!("Received:\n\t{:?}", converted_buffer);
    // }
    // stream.write(&[1])?;
    // Ok(())
}
