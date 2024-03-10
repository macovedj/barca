#[allow(warnings)]
mod bindings;

use crate::bindings::exports::macovedj::shapes::hashimap::{Guest, GuestHashimap};
use bindings::macovedj::shapes::hashimap::Hashimap as Unsorted;

struct Component;
struct Hashimap {
    map: Unsorted,
}

impl GuestHashimap for Hashimap {
    fn new() -> Self {
        Self {
            map: Unsorted::new(),
        }
    }

    fn get(&self, name: String) -> Option<String> {
        self.map.get(&name)
    }
    fn keys(&self) -> Vec<String> {
        let mut unsorted = self.map.keys().clone();
        unsorted.sort();
        unsorted
    }
}

impl Guest for Component {
    type Hashimap = Hashimap;
}

bindings::export!(Component with_types_in bindings);
