# Hello world in WASI Component Model with Wasmtime 

Currently, there are three ways to have a WASI Component Model:
1. Build a preview 1 `wasm` module and adapt it to a component in the guest (`guest-tools`)
2. Build a preview 1 `wasm` module, load it in as a binary in the host and adapt it to a component (`host-component`)
3. Build a preview 2 component via `cargo component` (`guest-cargo`)

In the Rust ecosystem, the third option is preferred. But all three are provided here as a proof of concept.
