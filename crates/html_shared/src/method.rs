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
