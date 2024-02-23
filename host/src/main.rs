use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!("hello-world" in "../wit/my-component.wit");

struct MyState {
    name: String,
    table: ResourceTable,
    wasi: WasiCtx,
}

impl HelloWorldImports for MyState {
    fn name(&mut self) -> wasmtime::Result<String> {
        Ok(self.name.clone())
    }
}

// Needed for wasmtime_wasi::preview2
impl WasiView for MyState {
    fn table(&self) -> &ResourceTable {
        &self.table
    }
    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    // let component = Component::from_file(&engine, "../guest/guest.component.wasm")?;
    let component = Component::from_file(
        &engine,
        "../guest-cargo/target/wasm32-wasi/debug/guest_cargo.wasm",
    )?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);

    // Bind wasi commmand world
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;
    // Binding host
    HelloWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let table = ResourceTable::new();

    let wasi = WasiCtxBuilder::new()
        // .inherit_stderr()
        // .inherit_stdin()
        .inherit_stdout()
        .inherit_stdio()
        .build();

    let mut store = Store::new(
        &engine,
        MyState {
            name: "me".to_string(),
            table,
            wasi,
        },
    );

    let (bindings, _) = HelloWorld::instantiate(&mut store, &component, &linker)?;

    // Here our `greet` function doesn't take any parameters for the component,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    bindings.call_greet(&mut store)?;

    Ok::<(), anyhow::Error>(())
}
