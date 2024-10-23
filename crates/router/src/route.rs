use std::fmt::Display;

use html_shared::method::HTTPMethod;

// pub struct Route {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Route(pub HTTPMethod, pub String);

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}
