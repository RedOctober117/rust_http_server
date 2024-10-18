use async_std::sync::RwLock;
use core::fmt;
use std::sync::Arc;
// use log::info;
use html_shared::header::Header;
use html_shared::method::HTTPMethod;
use html_shared::protocol::HTTPProtocol;
use html_shared::status::*;
use router::route::Route;
use router::router::Router;
use std::fs;
use std::path::Path;

use crate::errors::RequestGenerationError;
use crate::request::*;
use async_std;

pub const FILES_PATH: &str = "files/";

#[derive(Debug)]
pub struct ResponseMessage<'a> {
    status_line: StatusLine<'a>,
    headers_table: Vec<Header>,
    body: Option<String>,
}

impl<'a> ResponseMessage<'a> {
    pub async fn build_response(
        request: RequestMessage,
        router: Arc<RwLock<Router>>,
    ) -> Result<Self, RequestGenerationError> {
        let status_line_protocol = request.get_control_line().get_protocol();
        let mut status_line_code: Option<StatusCodeEnum> = None;

        let mut headers_table = vec![];
        let mut body = None;

        // Setup the files dir, just in case.
        let files_path = Path::new(FILES_PATH);
        if !files_path.exists() {
            fs::create_dir(files_path).expect("Could not create files directory");
        }

        let route = Route(
            request.get_control_line().get_method(),
            request.get_control_line().get_path().to_string(),
            // FILES_PATH.to_string(),
        );

        let router_reference = router.read_arc().await;
        // Route the path.
        let path = router_reference
            .get_abs_path(route)
            .unwrap_or("INVALID PATH");

        match request.get_control_line().get_method() {
            HTTPMethod::GET => {
                if let Ok(file) = fs::read_to_string(path) {
                    println!("Path already exists {:?}", path);
                    body = Some(file.clone());
                    status_line_code = Some(CODE200);
                    headers_table.push(Header::ContentType("text/html; charset=utf-8".to_string()));
                    headers_table.push(Header::ContentDisposition("inline".to_string()));
                    headers_table.push(Header::ContentLength(file.len().to_string()));
                } else {
                    println!("Path does not exist {:?}", path);
                    status_line_code = Some(CODE400);
                }
            }
            HTTPMethod::PUT => {
                // Check if file already exists, if so, set status code to 200 and write file
                // if not, set code to 201 and write file
                let contents = request.get_body();

                match fs::exists(path) {
                    Ok(true) => {
                        if fs::write(path, contents.unwrap()).is_err() {
                            println!("Path existed but could not write to path {:?}", path);
                            status_line_code = Some(CODE500);
                        } else {
                            status_line_code = Some(CODE200);
                        }
                    }
                    Ok(false) => {
                        if fs::write(path, contents.unwrap()).is_err() {
                            println!("Path did not exist and could not write to path {:?}", path);
                            status_line_code = Some(CODE501);
                        } else {
                            status_line_code = Some(CODE201);
                        }
                    }
                    Err(_) => {
                        println!("Error finding path {:?}", path);
                        status_line_code = Some(CODE500);
                    }
                }
            }
            _ => println!(
                "IMPLEMENT RESPONSE::HTTPMETHOD::METHOD {:?}",
                request.get_control_line().get_method()
            ),
        }

        Ok(Self {
            status_line: StatusLine::init(status_line_protocol, status_line_code.unwrap()),
            headers_table,
            body,
        })
    }
}

impl<'a> fmt::Display for ResponseMessage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_code = match self.status_line.status_code() {
            StatusCodeEnum::Code200(code) => code,
            StatusCodeEnum::Code201(code) => code,
            StatusCodeEnum::Code400(code) => code,
            StatusCodeEnum::Code404(code) => code,
            StatusCodeEnum::Code500(code) => code,
            StatusCodeEnum::Code501(code) => code,
        };

        let mut headers_as_string = String::new();
        for header in &self.headers_table {
            match header {
                Header::EMPTY => headers_as_string.push_str(&header.to_string()),
                Header::Accept(_) => headers_as_string.push_str(&header.to_string()),
                Header::UserAgent(_) => headers_as_string.push_str(&header.to_string()),
                Header::ContentType(_) => headers_as_string.push_str(&header.to_string()),
                Header::ContentLength(_) => headers_as_string.push_str(&header.to_string()),
                Header::Host(_) => headers_as_string.push_str(&header.to_string()),
                Header::AcceptLanguage(_) => headers_as_string.push_str(&header.to_string()),
                Header::AcceptEncoding(_) => headers_as_string.push_str(&header.to_string()),
                Header::Referer(_) => headers_as_string.push_str(&header.to_string()),
                Header::ContentDisposition(_) => headers_as_string.push_str(&header.to_string()),
            }
            headers_as_string.push_str("\r\n");
        }

        write!(
            f,
            "{} {}\r\n{}\r\n{}\r\n",
            self.status_line.protocol(),
            status_code,
            headers_as_string,
            match &self.body {
                Some(string) => string,
                None => "",
            }
        )
    }
}

#[derive(Debug)]
pub struct StatusLine<'a> {
    protocol: HTTPProtocol,
    status_code: StatusCodeEnum<'a>,
}

impl<'a> StatusLine<'a> {
    pub fn init(protocol: HTTPProtocol, status_code: StatusCodeEnum<'a>) -> Self {
        Self {
            protocol,
            status_code,
        }
    }

    pub fn status_code(&self) -> StatusCodeEnum {
        self.status_code
    }

    pub fn protocol(&self) -> HTTPProtocol {
        self.protocol
    }
}
