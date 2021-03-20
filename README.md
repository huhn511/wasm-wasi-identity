## Wasm/WASI Example

## WASI Test

https://docs.wasmtime.dev/wasm-rust.html

```bash
cargo install cargo-wasi
```

```bash
cargo wasi run
```

```bash
wasmtime target/wasm32-wasi/debug/hello-world.wasm
wasmer target/wasm32-wasi/debug/hello-world.wasm
```

## Wasm

```bash
cargo build --target wasm32-unknown-unknown
``` 

```bash
wasmtime target/wasm32-unknown-unknown/debug/hello-world.wasm
wasmer target/wasm32-unknown-unknown/debug/hello-world.wasm
``` 
