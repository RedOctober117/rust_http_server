use crate::route::Route;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Router {
    route_map: HashMap<Route, String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            route_map: HashMap::new(),
        }
    }

    pub fn connect_route(&mut self, canonical_route: Route, absolute_path: String) {
        self.map_director(&absolute_path);
        self.route_map.insert(canonical_route, absolute_path);
    }

    pub fn get_abs_path(&self, route: Route) -> Option<&str> {
        match self.route_map.get(&route) {
            Some(route) => Some(route),
            None => None,
        }
    }

    pub fn disconnect_route(&mut self, route: &Route) -> Option<String> {
        self.route_map.remove(route)
    }

    fn map_director(&mut self, from: &String) {
        let mut initial_path = PathBuf::from(from);
        if !initial_path.is_dir() {
            let popped = initial_path.pop();
            println!("{:?}", popped);
        }
        let _ = fs::read_dir(initial_path).map(|item| {
            println!("{:?}", item);
        });
    }
}
