use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Route {
    route_map: HashMap<String, PathBuf>,
}

impl Route {
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
