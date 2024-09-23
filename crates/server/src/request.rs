// A "message" consists of the following:
// - control data to describe and route the message,
// - a headers lookup table of name/value pairs for extending that control data
//      and conveying additional information about the sender, message,
//      content, or context,
// - a potentially unbounded stream of content
// - a trailers lookup table of name/value pairs for communicating information
//      obtained while sending the content.

pub struct RequestMessage {
    control_data: ControlData,
    headers_table: Option<Vec<Header>>,
    message: Option<String>,
}

pub struct ControlData {
    method: HTTPMethod,
    protocol: HTTPProtocol,
}

pub enum HTTPMethod {
    GET { req: Option<String>, res: String },
    HEAD { req: Option<String> },
    POST { req: String, res: String },
    PUT { req: String, res: String },
    DELETE { req: Option<String>, res: String },
    CONNECT { req: Option<String>, res: String },
    OPTIONS { req: Option<String>, res: String },
    TRACE { res: String },
    PATCH { req: Option<String>, res: String },
}

pub enum HTTPProtocol {
    HTTP_1_1,
}

pub enum Header {
    UserAgent(String),
}
