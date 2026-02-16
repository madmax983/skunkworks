use anyhow::{Result, Context};
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

pub fn run(wasm_bytes: &[u8]) -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    // Add WASI to linker
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .build();

    let mut store = Store::new(&engine, wasi);
    let module = Module::new(&engine, wasm_bytes)?;

    // Instantiate
    linker.module(&mut store, "", &module)?;

    let instance = linker.instantiate(&mut store, &module)
        .context("Failed to instantiate module")?;

    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")
        .context("Module does not export `_start` function")?;

    start.call(&mut store, ())?;

    Ok(())
}
