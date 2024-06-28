# ft-stripe

## Why is this create in this repo and not another repo?

We are currently implementing `ft-stripe` in pure wasm, but we have considered that
may be not optimal as every single wasm file that depends on stripe will have to include
the full stripe library. The library is pretty large.

We can do better by trying to put ft-stripe related function in `host` itself, and
exposing it as host functions. It is not clear right now that this is the way to go, or
optimizing the stripe library with features etc is the way to go to do this optimally,
so we are keeping this crate here for now.