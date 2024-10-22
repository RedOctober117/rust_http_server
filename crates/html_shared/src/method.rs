use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HTTPMethod::EMPTY => "EMPTY",
                HTTPMethod::GET => "GET",
                HTTPMethod::POST => "POST",
                HTTPMethod::HEAD => "HEAD",
                HTTPMethod::PUT => "PUT",
                HTTPMethod::DELETE => "DELETE",
                HTTPMethod::CONNECT => "CONNECT",
                HTTPMethod::OPTIONS => "OPTIONS",
                HTTPMethod::TRACE => "TRACE",
                HTTPMethod::PATCH => "PATCH",
            }
        )
    }
}
