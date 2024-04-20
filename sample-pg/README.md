# Developer README

```shell
# one time
rustup target add wasm32-unknown-unknown

# everytime you modify code
cargo build --target wasm32-unknown-unknown
```

# Numbers

```
build-sample-wasm &&  WASMTIME_BACKTRACE_DETAILS=1 cargo run

module created in 345.300875ms
wasm: id: 1, name: amit
10 in 353.768125ms/8.256292ms

ls -lh ../sample-wasm/target/wasm32-unknown-unknown/release/sample_wasm.wasm
-rwxr-xr-x@ 1 amitu  staff   292K Mar  7 11:42 sample_wasm.wasm
```

So our simple project is `292K` and it takes `345ms` to compile, and `8.25ms`
to execute.

## With Serialization

Serializing the module takes `5m` and loading serialised module from disc take
`0.5ms` (`585.083µs`). The serialised file size is `661K`.

## GZip

GZip brings the wasm file size down from `292K` to `115K`, the serialized module
snapshot from `661K` to `247K`.