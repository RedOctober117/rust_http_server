use crate::protocol::HTTPProtocol;

pub const CODE200: StatusCodeEnum<'_> = StatusCodeEnum::Code200("200 Ok");
pub const CODE201: StatusCodeEnum<'_> = StatusCodeEnum::Code201("201 Created");
pub const CODE400: StatusCodeEnum<'_> = StatusCodeEnum::Code400("400 Bad Request");
pub const CODE404: StatusCodeEnum<'_> = StatusCodeEnum::Code404("404 Not Found");
pub const CODE500: StatusCodeEnum<'_> = StatusCodeEnum::Code500("500 Internal Server Error");
pub const CODE501: StatusCodeEnum<'_> = StatusCodeEnum::Code501("501 Internal Server Error");

#[derive(Debug, Clone, Copy)]
pub enum StatusCodeEnum<'a> {
    Code200(&'a str),
    Code201(&'a str),
    Code400(&'a str),
    Code404(&'a str),
    Code500(&'a str),
    Code501(&'a str),
}
