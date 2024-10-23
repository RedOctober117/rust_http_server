// A "message" consists of the following:
// - control data to describe and route the message,
// - a headers lookup table of name/value pairs for extending that control data
//      and conveying additional information about the sender, message,
//      content, or context,
// - a potentially unbounded stream of content
// - a trailers lookup table of name/value pairs for communicating information
//      obtained while sending the content.

use core::str;
use html_shared::{header::Header, method::HTTPMethod, protocol::HTTPProtocol};
use std::{collections::BTreeSet, fmt::Display};

use crate::errors::MessageParseError;

#[derive(Debug, Clone)]
pub struct RequestMessage {
    control_data: ControlData,
    headers_table: Option<BTreeSet<Header>>,
    message: Option<String>,
}

impl RequestMessage {
    pub fn parse_request(request: &[u8]) -> Result<Self, MessageParseError> {
        let mut control_data = ControlData {
            method: HTTPMethod::EMPTY,
            path: String::new(),
            protocol: HTTPProtocol::Http1_1,
        };

        let mut headers: BTreeSet<Header> = BTreeSet::new();
        let mut message: Option<String> = None;

        let request_as_string = str::from_utf8(request).unwrap().trim_matches('\0');
        if request_as_string.is_empty() {
            return Err(MessageParseError);
        }

        let sections: Vec<&str> = request_as_string.split("\r\n\r\n").collect();
        let control_and_header: Vec<&str> = sections[0].split("\r\n").collect();

        // CONTROL DATA
        let control_split: Vec<&str> = control_and_header[0].split(" ").collect();
        match control_split[0].trim() {
            "GET" => control_data.method = HTTPMethod::GET,
            "POST" => control_data.method = HTTPMethod::POST,
            "HEAD" => control_data.method = HTTPMethod::HEAD,
            "PUT" => control_data.method = HTTPMethod::PUT,
            "DELETE" => control_data.method = HTTPMethod::DELETE,
            "CONNECT" => control_data.method = HTTPMethod::CONNECT,
            "OPTIONS" => control_data.method = HTTPMethod::OPTIONS,
            "TRACE" => control_data.method = HTTPMethod::TRACE,
            "PATCH" => control_data.method = HTTPMethod::PATCH,
            _ => println!("IMPLEMENT REQUEST::CONTROL::METHOD {}", control_split[0]),
        }

        control_data.path = control_split[1].to_string();

        match control_split[2].trim() {
            "HTTP/1.1" => control_data.protocol = HTTPProtocol::Http1_1,
            _ => println!(
                "IMPLEMENT REQUEST::CONTROL::HTTPProtocol {}",
                control_split[2]
            ),
        }

        let mut header_items = vec![];
        if control_and_header.len() > 1 {
            //for <item> in control_and_header.iter().skip(1)
            for header in control_and_header.iter().skip(1) {
                header_items.push(header);
            }
        }

        // HEADER DATA
        if control_and_header.len() > 1 {
            for header in header_items {
                let split: Vec<_> = header.split(": ").collect();
                println!("split 0: {}", header);

                match split[0] {
                    "User-Agent" => _ = headers.insert(Header::UserAgent(split[1].to_string())),
                    "Content-Type" => _ = headers.insert(Header::ContentType(split[1].to_string())),
                    "Content-Length" => {
                        _ = headers.insert(Header::ContentLength(split[1].to_string()))
                    }
                    "Host" => _ = headers.insert(Header::Host(split[1].to_string())),
                    "Accept" => _ = headers.insert(Header::Accept(split[1].to_string())),
                    "Accept-Language" => {
                        _ = headers.insert(Header::AcceptLanguage(split[1].to_string()))
                    }
                    "Accept-Encoding" => {
                        _ = headers.insert(Header::AcceptEncoding(split[1].to_string()))
                    }
                    "Referer" => _ = headers.insert(Header::Referer(split[1].to_string())),
                    // &_ => continue,
                    &_ => println!("IMPLEMENT REQUEST::HEADERS::HEADER {}", split[0]),
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

    pub fn get_control_line(&self) -> &ControlData {
        &self.control_data
    }

    pub fn get_body(&self) -> Option<&str> {
        if let Some(body) = &self.message {
            Some(body.as_str())
        } else {
            None
        }
    }

    pub fn get_headers_table(&self) -> Option<&BTreeSet<Header>> {
        match &self.headers_table {
            Some(table) => Some(&table),
            None => None,
        }
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

#[derive(Debug, Clone)]
pub struct ControlData {
    method: HTTPMethod,
    path: String,
    protocol: HTTPProtocol,
}

impl ControlData {
    pub fn get_method(&self) -> HTTPMethod {
        self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_protocol(&self) -> HTTPProtocol {
        self.protocol
    }
}
