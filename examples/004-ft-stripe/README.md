# stripe-example - fastn + wasm

This is a simple example of how to use fastn with wasm. To run this example, you need
to have fastn installed. You can start the server using:

```shell
# make sure you are in hello-world directory
fastn serve --port 8001
### Server Started ###
Go to: http://127.0.0.1:8001
```

This runs fastn server on port 8001. You can also omit the port number to run it on the
first free port starting 8000.

# Building WebAssembly

```shell
cargo build --target wasm32-unknown-unknown --release

# you may have to run `rustup target add wasm32-unknown-unknown` to add the target
```

The above command will build the wasm file in `../../target/wasm32-unknown-unknown/release/stripe_example.wasm`.
The target folder in `../../` because this example is part of a cargo workspace, if you are
following along, the target folder will be in the current folder.

Let's copy the wasm file to current folder:

```shell
cp ../../target/wasm32-unknown-unknown/release/stripe_example.wasm .
```

You can use the `build.sh` script to build the wasm file and copy it to the current folder:

```shell
sh build.sh
```

# Restart the server if you modify `FASTN.ftd`

Every time you modify `FASTN.ftd` file, you need to restart the server to see the changes.
