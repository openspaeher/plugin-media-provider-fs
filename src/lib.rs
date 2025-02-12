#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn test() {
        print!("hello world");
    }
}

bindings::export!(Component with_types_in bindings);