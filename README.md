# ft-sdk

[docs.rs/ft-sdk](https://docs.rs/ft-sdk)

This crate can only be used when compiled to wasm, and wasm is run by
[www.fifthtry.com](https://www.fifthtry.com), or by `clift`, the command
line tool to use help developers build FifthTry Apps or when self-hosting
FifthTry Apps.

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
