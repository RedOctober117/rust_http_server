use std::fmt;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
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

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UserAgent(_), Self::UserAgent(_)) => true,
            (Self::ContentType(_), Self::ContentType(_)) => true,
            (Self::ContentLength(_), Self::ContentLength(_)) => true,
            (Self::Host(_), Self::Host(_)) => true,
            (Self::Accept(_), Self::Accept(_)) => true,
            (Self::AcceptLanguage(_), Self::AcceptLanguage(_)) => true,
            (Self::AcceptEncoding(_), Self::AcceptEncoding(_)) => true,
            (Self::Referer(_), Self::Referer(_)) => true,
            (Self::ContentDisposition(_), Self::ContentDisposition(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
