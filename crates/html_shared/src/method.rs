#[derive(Debug, Clone, Copy, PartialEq)]
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
