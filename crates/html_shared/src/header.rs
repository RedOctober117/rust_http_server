use std::fmt;

#[derive(Debug, Clone)]
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
    ContentDisposition(String),
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Header::EMPTY => "EMPTY".to_string(),
                Header::UserAgent(value) => format!("User-Agent: {}", value),
                Header::ContentType(value) => format!("Content-Type: {}", value),
                Header::ContentLength(value) => format!("Content-Length: {}", value),
                Header::Host(value) => format!("Host: {}", value),
                Header::Accept(value) => format!("Accept: {}", value),
                Header::AcceptLanguage(value) => format!("Accept-Language: {}", value),
                Header::AcceptEncoding(value) => format!("Accept-Encoding: {}", value),
                Header::Referer(value) => format!("Referer: {}", value),
                Header::ContentDisposition(value) => format!("Content-Disposition: {}", value),
            }
        )
    }
}
