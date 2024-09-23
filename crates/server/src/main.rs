// use server::{HttpSchemeEnum, Tag, Tokenizer, Uri};

use request::{MessageParseError, RequestMessage};

pub mod request;

fn main() -> Result<(), MessageParseError> {
    let buffer = r##"POST /echo HTTP/1.1
Host: reqbin.com
Content-Type: application/x-www-form-urlencoded
Content-Length: 0"##;
    // r##"HTTP/1.1 405 METHOD NOT ALLOWED
    // Date: Mon, 23 Sep 2024 18:07:48 GMT
    // Content-Type: text/html
    // Transfer-Encoding: chunked
    // Connection: keep-alive
    // Allow: GET, HEAD, OPTIONS
    // CF-Cache-Status: DYNAMIC
    // Report-To: {"endpoints":[{"url":"https:\/\/a.nel.cloudflare.com\/report\/v4?s=u8XSZXb33szAPdw8xHCfzZsqbNw%2B%2BagS52Hf83mnS%2BtqPmGRKEkrKyAIFtbrxNIzr98WD0gtvCI%2FiA95uvQanQ6MZaAY1Qvsgx1FlKhNBTH0kSSOPQs6sN%2FacSs%3D"}],"group":"cf-nel","max_age":604800}
    // NEL: {"success_fraction":0,"report_to":"cf-nel","max_age":604800}
    // referrer-policy: same-origin
    // x-content-type-options: nosniff
    // x-frame-options: SAMEORIGIN
    // x-xss-protection: 1; mode=block
    // Access-Control-Allow-Origin: https://reqbin.com
    // Server: cloudflare
    // CF-RAY: 8c7c79f93c3b4269-EWR

    // <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final//EN"><title>405 Method Not Allowed</title> <h1>Method Not Allowed</h1> <p>The method is not allowed for the requested URL.</p> <script>(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement('script');d.innerHTML="window.__CF$cv$params={r:'8c7c79f93c3b4269',t:'MTcyNzExNDg2OC4wMDAwMDA='};var a=document.createElement('script');a.nonce='';a.src='/cdn-cgi/challenge-platform/scripts/jsd/main.js';document.getElementsByTagName('head')[0].appendChild(a);";b.getElementsByTagName('head')[0].appendChild(d)}}if(document.body){var a=document.createElement('iframe');a.height=1;a.width=1;a.style.position='absolute';a.style.top=0;a.style.left=0;a.style.border='none';a.style.visibility='hidden';document.body.appendChild(a);if('loading'!==document.readyState)c();else if(window.addEventListener)document.addEventListener('DOMContentLoaded',c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);'loading'!==document.readyState&&(document.onreadystatechange=e,c())}}}})();</script>"##.as_bytes();

    let result = RequestMessage::parse_request(buffer.as_bytes())?;
    println!("Parsed\n\n{}\n\nto\n\n{}", buffer, result);
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
