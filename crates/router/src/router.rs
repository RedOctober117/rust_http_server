use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Router {
    route_map: HashMap<String, PathBuf>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            route_map: HashMap::<String, PathBuf>::new(),
        }
    }
    pub fn connect(&mut self, html_path: &str, abs_path: PathBuf) {
        self.route_map.insert(html_path.to_string(), abs_path);
    }

    pub fn match_path(&self, html_path: &str) -> Option<&str> {
        match self.route_map.get(html_path) {
            Some(value) => value.to_str(),
            None => None,
        }
    }

    pub fn disconnect(&mut self, html_path: &str) -> Option<PathBuf> {
        self.route_map.remove(html_path)
    }
}

// pub struct Route {
//     route_method:
// }
