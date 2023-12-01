use std::cell::RefCell;

use super::view::View;
use ahash::{HashMap, HashMapExt};
use ratatui::Frame;

pub struct Navigator {
    routes: HashMap<String, RefCell<Route>>,
    stack: Vec<String>,
}

impl Navigator {
    pub fn new(routes: Vec<Route>) -> Self {
        let mut route_map = HashMap::<String, RefCell<Route>>::with_capacity(routes.len());
        let mut stack = Vec::<String>::with_capacity(10);
        stack.push(routes[0].route.to_owned());
        for route in routes {
            route_map.insert(route.route.clone(), RefCell::new(route));
        }
        Navigator {
            routes: route_map,
            stack,
        }
    }

    pub fn run(&mut self, frame: &mut Frame<'_>) {
        if let Some(route) = self.routes.get(&(self.stack[self.stack.len() - 1])) {
            let _ = (route.borrow_mut()).view.draw(frame);
        }
    }

    /// Navigate to a specified `Route`
    /// The navigator displays the `View` at the end of the history `stack`
    pub fn route(&mut self, route: String) {
        if let Some(_result) = self.routes.get(&route) {
            self.stack.push(route);
            if self.stack.len() > 10 {
                self.stack.remove(0);
            }
        }
    }

    pub fn back(&mut self) {
        self.stack.pop();
    }
}

pub struct Route {
    pub view: Box<dyn View>,
    pub route: String,
    pub id: u8,
}

impl Route {
    pub fn new(route: &str, view: Box<dyn View>) -> Self {
        Route { route: route.to_string(), view, id: 2 }
    }
}
