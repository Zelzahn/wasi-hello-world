mod bindings;

use bindings::Guest;
use crate::bindings::name;

struct Component;

impl Guest for Component {
       fn greet() {
        println!("Hello {} from the guest", name());
    }
}
