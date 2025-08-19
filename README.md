# ft-sdk
Rust SDK for extending backend functionality of
[fastn](https://github.com/fastn-stack/fastn/) using WASM modules.

[docs.rs/ft-sdk](https://docs.rs/ft-sdk)

> [!NOTE]
> This crate should only be used when compiled to wasm.
---

## Example

```rust
// lib.rs

// An http handler available at /<mountpoint>/hello-world/
#[ft_sdk::processor]
fn hello_world(path: ft_sdk::Path) -> ft_sdk::processor::Result {
    ft_sdk::println!("params: {path}");
    // Return a JSON string as response. Complex json with `serde_json::json!`
    // or any Serializable struct is supported.
    ft_sdk::processor::json("and this is coming from wasm!")
}
```

Compile the rust project with target `wasm32-unknown-unknown` and copy the
`.wasm` file into your fastn package.

You can then mount this module by adding the following to your `FASTN.ftd`
file:

```ftd
-- import: fastn

-- fastn.package: hello-world

-- fastn.url-mappings:

/wasm/* -> wasm+proxy://hello_world.wasm/*
```

Now any `.ftd` file can make request to `/wasm/hello-world/` and it will be
handled by the `hello_world` function in the wasm module:

```ftd
-- import: fastn/processors as pr

-- ftd.text: this is in ftd file
-- ftd.text: $message

-- string message:
$processor$: pr.http
url: /wasm/hello-world/
```

See [./examples/](./examples/) for more examples including form handling and
interacting with database using the
[diesel](https://github.com/diesel-rs/diesel) crate.

## Maintenance Note

Run `scripts/check.sh` before release.

## ft-stripe

This project provides integration with Stripe HTTP APIs. It is based on the
[async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
project, with its own implementation of a Client to call Stripe APIs.

### License

This project includes many source files copied from
[async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7).
We acknowledge and are grateful to the developers of async-stripe for their
contributions to open source.

Licensed under either of

- Apache License, Version 2.0,
  ([LICENSE-APACHE](https://github.com/arlyon/async-stripe/blob/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7/LICENSE-APACHE)
  or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](https://github.com/arlyon/async-stripe/blob/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7/LICENSE-MIT)
  or
  https://opensource.org/licenses/MIT) at your option.
