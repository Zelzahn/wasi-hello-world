wit_bindgen::generate!({
    path: "../wit",
    world: "hello-world",
    exports: {
        world: Component
    }
});

struct Component;

impl Guest for Component {
    fn greet() {
        println!("Hello {} from the guest", name());
    }
}

