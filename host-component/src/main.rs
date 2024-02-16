use anyhow::Context;
use std::{fs, path::Path};
use wit_component;

use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Result, Store,
};

// Generate bindings of the guest and host components.
bindgen!("hello-world" in "../wit/my-component-alt.wit");

struct HostComponent;

// Implmentation of the host interface defined in the wit file.
impl host::Host for HostComponent {
    fn name(&mut self) -> wasmtime::Result<String> {
        Ok("host with wit-component".to_string())
    }
}

struct MyState {
    host: HostComponent,
}

/// This function is only needed until rust can natively output a component.
///
/// Generally embeddings should not be expected to do this programatically, but instead
/// language specific tooling should be used, for example in Rust `cargo component`
/// is a good way of doing that: https://github.com/bytecodealliance/cargo-component
///
/// In this example we convert the code here to simplify the testing process and build system.
fn convert_to_component(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let bytes = &fs::read(&path).context("failed to read input file")?;
    let reactor_bytes = &fs::read("./wasi_snapshot_preview1.reactor.wasm").context("failed to read adapter file")?;
    wit_component::ComponentEncoder::default()
        .module(&bytes)?
        .adapter("wasi_snapshot_preview1", reactor_bytes)?
        .encode()
}

fn main() -> Result<()> {
    // Create an engine with the component model enabled (disabled by default).
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    // NOTE: In the original example (https://github.com/bytecodealliance/wasmtime/blob/main/examples/component) the target is wasm32-unknown-unknown, we use wasm32-wasi. 
    // For this wasi preview2 needs to be enabled in the component model, doing this results in
    // the following error:
    // Error: import `wasi:cli/environment@0.2.0` has the wrong type
    //
    // Caused by:
    //  0: instance export `get-environment` has the wrong type
    //  1: expected func found nothing
    let component = convert_to_component("../guest-tools/target/wasm32-wasi/debug/guest.wasm")?;

    // Create our component and call our generated host function.
    let component = Component::from_binary(&engine, &component)?;
    let mut store = Store::new(
        &engine,
        MyState {
            host: HostComponent {},
        },
    );
    let mut linker = Linker::new(&engine);
    host::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    let (hello_world, _instance) = HelloWorld::instantiate(&mut store, &component, &linker)?;
    hello_world.call_greet(&mut store)?;
    Ok(())
}
