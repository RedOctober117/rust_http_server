// A "message" consists of the following:
// - control data to describe and route the message,
// - a headers lookup table of name/value pairs for extending that control data
//      and conveying additional information about the sender, message,
//      content, or context,
// - a potentially unbounded stream of content
// - a trailers lookup table of name/value pairs for communicating information
//      obtained while sending the content.

use core::str;
use std::fmt::Display;

#[derive(Debug)]
pub struct RequestMessage {
    control_data: ControlData,
    headers_table: Option<Vec<Header>>,
    message: Option<String>,
}

impl RequestMessage {
    pub fn parse_request(request: &[u8]) -> Result<Self, MessageParseError> {
        let mut control_data = ControlData {
            method: HTTPMethod::EMPTY,
            path: String::new(),
            protocol: HTTPProtocol::Http1_1,
        };

        let mut headers: Vec<Header> = vec![];
        let mut message: Option<String> = None;

        let request_as_string = str::from_utf8(request).unwrap();
        let sections: Vec<&str> = request_as_string.split("\x0A\x0A").collect();
        println!("{:?}", sections);
        let control_and_header: Vec<&str> = sections[0].split("\x0A").collect();

        // CONTROL DATA
        let control_split: Vec<&str> = control_and_header[0].split(" ").collect();
        match control_split[0] {
            "GET" => control_data.method = HTTPMethod::GET,
            "POST" => control_data.method = HTTPMethod::POST,
            "HEAD" => control_data.method = HTTPMethod::HEAD,
            "PUT" => control_data.method = HTTPMethod::PUT,
            "DELETE" => control_data.method = HTTPMethod::DELETE,
            "CONNECT" => control_data.method = HTTPMethod::CONNECT,
            "OPTIONS" => control_data.method = HTTPMethod::OPTIONS,
            "TRACE" => control_data.method = HTTPMethod::TRACE,
            "PATCH" => control_data.method = HTTPMethod::PATCH,
            _ => todo!(),
        }

        control_data.path = control_split[1].to_string();

        match control_split[2] {
            "HTTP/1.1" => control_data.protocol = HTTPProtocol::Http1_1,
            _ => todo!(),
        }

        let mut header_items = vec![];
        if control_and_header.len() > 1 {
            for i in 1..control_and_header.len() {
                header_items.push(&control_and_header[i]);
            }
        }

        // HEADER DATA
        if control_and_header.len() > 1 {
            for header in header_items {
                let split: Vec<_> = header.split(": ").collect();
                println!("{:?}", split[0]);
                match split[0] {
                    "User-Agent" => headers.push(Header::UserAgent(split[1].to_string())),
                    "Content-Type" => headers.push(Header::ContentType(split[1].to_string())),
                    "Content-Length" => headers.push(Header::ContentLength(split[1].to_string())),
                    "Host" => headers.push(Header::Host(split[1].to_string())),
                    "Accept" => headers.push(Header::Accept(split[1].to_string())),
                    "Accept-Language" => headers.push(Header::AcceptLanguage(split[1].to_string())),
                    "Accept-Encoding" => headers.push(Header::AcceptEncoding(split[1].to_string())),
                    "Referer" => headers.push(Header::Referer(split[1].to_string())),
                    &_ => todo!(),
                }
            }
        }

        if sections.len() > 1 {
            let body_split = sections[1];
            if !body_split.is_empty() {
                message = Some(body_split.to_string());
            }
        }

        Ok(Self {
            control_data,
            headers_table: if !headers.is_empty() {
                Some(headers)
            } else {
                None
            },
            message,
        })
    }
}

impl Display for RequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Control Data: {:?}\nHeaders: {:?}\nBody: {:?}",
            self.control_data, self.headers_table, self.message
        )
    }
}

#[derive(Debug)]
pub struct ControlData {
    method: HTTPMethod,
    path: String,
    protocol: HTTPProtocol,
}

#[derive(Debug)]
pub enum HTTPMethod {
    EMPTY,
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug)]
pub enum HTTPProtocol {
    EMPTY,
    Http1_1,
}

#[derive(Debug)]
pub enum Header {
    EMPTY,
    UserAgent(String),
    ContentType(String),
    ContentLength(String),
    Host(String),
    Accept(String),
    AcceptLanguage(String),
    AcceptEncoding(String),
    Referer(String),
}

#[derive(Debug)]
pub struct MessageParseError;
