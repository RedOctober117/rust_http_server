use crate::route::Route;
use html_shared::method::HTTPMethod;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

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

    /// Connects passed Route and all other items in the absolute_paths
    /// containing directory.
    pub fn connect_single_route(&mut self, canonical_route: Route, absolute_path: String) {
        println!("Connecting {} to {}", canonical_route, absolute_path);
        self.route_map.insert(canonical_route, absolute_path);
    }

    pub fn connect_recursive_routes(&mut self, canonical_route: Route, absolute_path: String) {
        let target = Path::new(&absolute_path);

        if target.is_dir() {
            println!("target is dir");
            _ = self.map_directory(canonical_route.1.clone(), target);
        } else {
            let mut target_container = PathBuf::from(target);
            target_container.pop();
            println!(
                "target is not dir, popped up to container {:?}",
                target_container
            );
            _ = self.map_directory(canonical_route.1.clone(), &target_container);
        }
        self.connect_single_route(canonical_route.clone(), absolute_path.clone());
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

    fn map_directory(
        &mut self,
        canonical_route_prefix: String,
        current_target: &Path,
    ) -> io::Result<()> {
        // for each item in dir, bind item, and if dir, recursive map dir
        let dir_items = fs::read_dir(current_target)?;
        for entry in dir_items {
            let entry = entry?;
            let new_canonical_route_prefix = format!(
                "{}{}",
                canonical_route_prefix,
                entry.file_name().to_str().unwrap()
            );
            if entry.path().is_dir() {
                self.connect_single_route(
                    Route(HTTPMethod::GET, new_canonical_route_prefix.clone()),
                    entry.path().to_str().unwrap().to_string(),
                );
                self.map_directory(new_canonical_route_prefix, entry.path().as_path())?;
            } else {
                self.connect_single_route(
                    Route(HTTPMethod::GET, new_canonical_route_prefix.clone()),
                    entry.path().to_str().unwrap().to_string(),
                );
            }
        }
        Ok(())
    }
}
