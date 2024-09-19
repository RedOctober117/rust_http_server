use std::net::TcpListener;

use server::{GlobalState, HttpRequest, Tag, Tokenizer, Uri};

fn main() {
    let test_string = String::from("https://telemakos.io:443/test");
    let mut tokenizer = Tokenizer::new(test_string.clone());
    let mut tokens = vec![];
    println!("Tokenizing {} . . .", test_string);
    loop {
        let next = tokenizer.next();

        tokens.push(next);
        if tokenizer.state() == GlobalState::EndOfURI {
            break;
        }
    }

    for token in tokens {
        println!(
            "{:?}: {}",
            token.tag(),
            &test_string[token.location().start()..token.location().end()]
        )
    }
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
