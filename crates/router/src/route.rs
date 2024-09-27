use html_shared::method::HTTPMethod;

// pub struct Route {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Route(pub HTTPMethod, pub String);
