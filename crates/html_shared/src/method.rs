#[derive(Debug, Clone, Copy)]
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
