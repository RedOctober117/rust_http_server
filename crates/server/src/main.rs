use server::{HttpSchemeEnum, Tag, Tokenizer, Uri};

fn main() {
    let test_uri = String::from("https://telemakos.io/test?test_query=#fragment-here");
    let mut tokenizer = Tokenizer::new(test_uri.clone());

    println!("Tokenizing . . . \n{}", tokenizer);

    println!("Parsing {} . . .", test_uri);

    println!("{}", Uri::parse_tokens(&mut tokenizer));

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
