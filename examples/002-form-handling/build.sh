# how to install rustup: https://rust-lang.github.io/rustup/installation/index.html
# rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release
# make sure to change the name of the wasm file to match the name in Cargo.toml
cp ../../target/wasm32-unknown-unknown/release/form_handling.wasm .