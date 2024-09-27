use crate::route::Route;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Router {
    route_map: HashMap<Route, PathBuf>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            route_map: HashMap::<Route, PathBuf>::new(),
        }
    }
    pub fn connect(&mut self, route: Route, abs_path: PathBuf) {
        self.route_map.insert(route, abs_path);
    }

    pub fn route(&self, route: Route) -> Option<&str> {
        match self.route_map.get(&route) {
            Some(value) => value.to_str(),
            None => None,
        }
    }

    pub fn disconnect(&mut self, route: &Route) -> Option<PathBuf> {
        self.route_map.remove(route)
    }
}
