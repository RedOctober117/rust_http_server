use html_shared::method::HTTPMethod;

pub struct Route {
    methods: Vec<HTTPMethod>,
    // Canonical path means the path representation in the URI.
    canonical_path: String,
}

impl Route {
    /// This fn will always include the GET method by default.
    pub fn init(canonical_path: String) -> Self {
        Self {
            methods: vec![HTTPMethod::GET],
            canonical_path,
        }
    }

    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }

    pub fn add_method(&mut self, method: HTTPMethod) {
        if !self.methods.contains(&method) {
            self.methods.push(method);
        }
    }
}
