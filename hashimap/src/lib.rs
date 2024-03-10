use std::collections::HashMap;
#[allow(warnings)]
mod bindings;

use bindings::exports::barca::shapes::hashimap::{Guest, GuestHashimap};

struct Component;

struct Hashimap {
    data: HashMap<String, String>,
}

impl GuestHashimap for Hashimap {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("Danny".to_string(), "developer".to_string());
        map.insert("Calvin".to_string(), "programmer".to_string());
        map.insert("Oscar".to_string(), "engineer".to_string());
        Self { data: map }
    }
    fn get(&self, name: String) -> Option<String> {
        self.data.get(&name).cloned()
    }
    fn keys(&self) -> Vec<String> {
        self.data.keys().into_iter().map(|s| s.to_owned()).collect()
    }
}

impl Guest for Component {
    type Hashimap = Hashimap;
}

bindings::export!(Component with_types_in bindings);
