use std::fmt::Display;

pub struct HttpRequest {
    method: HttpMethodEnum,
    target: Uri,
}

pub enum HttpMethodEnum {
    GET,
    HEAD,
    POST,
}

#[derive(Debug)]
pub enum HttpSchemeEnum {
    HTTP,
    HTTPS,
}

#[derive(Debug)]
pub struct Uri {
    scheme: HttpSchemeEnum,
    host: String,
    port: u16,
    query: Option<[u8; 8000]>,
}

impl Uri {
    /// Shittiest parser know to man. Christ, just get a whiteboard and figure
    /// it out man.
    pub fn parse_buffer(buffer: &[u8; 8000]) -> Self {
        let mut buffer_as_string = String::new();
        for byte in buffer {
            if byte.clone() as char != '\0' {
                buffer_as_string.push(*byte as char);
            }
        }

        let scheme_split: Vec<_> = buffer_as_string.split("://").collect();
        let host_and_port_split: Vec<_> = scheme_split[1].split("/").collect();

        let scheme = match scheme_split[0] {
            "http" => HttpSchemeEnum::HTTP,
            "https" => HttpSchemeEnum::HTTPS,
            &_ => panic!("Couldn't parse URI"),
        };
        let host: String = String::from(host_and_port_split[0]);
        let port: u16 = match host_and_port_split[0].find(":") {
            Some(_) => host_and_port_split[0]
                .split(":")
                .nth(1)
                .unwrap()
                .parse()
                .ok()
                .unwrap(),
            None => match scheme {
                HttpSchemeEnum::HTTP => 80,
                HttpSchemeEnum::HTTPS => 443,
            },
        };
        // let query: Option<String> = None;

        Self {
            scheme,
            host,
            port,
            query: None,
        }

        // if let Some(scheme) = buffer_as_string.find("https") {
        //     scheme =
        // }
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.scheme, self.host, self.port)
    }
}
