use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum HTTPProtocol {
    EMPTY,
    Http1_1,
}

impl fmt::Display for HTTPProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HTTPProtocol::EMPTY => "EMPTY",
                HTTPProtocol::Http1_1 => "HTTP/1.1",
            }
        )
    }
}
