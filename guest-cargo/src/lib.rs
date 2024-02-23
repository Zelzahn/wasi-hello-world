mod bindings;

use crate::bindings::name;
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn greet() {
        println!("Hello {} from the guest", name());
    }
}
