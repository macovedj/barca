#[allow(warnings)]
mod bindings;

use bindings::macovedj::shapes::hashimap::Hashimap;

fn main() {
    let map = Hashimap::new();

    let keys = map.keys();
    for key in keys {
        let val = map.get(&key);
        dbg!(key, val);
    }
}
