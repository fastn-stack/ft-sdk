# ChangeLog

## 30th May 2024

### ft-sdk: `0.1.4`

- added `ft_sdk::auth_provider::updated_user()`
- `ft_sdk::auth_provider::create_user()` does not return session id, but the user id. also it  
  does not automatically log user in, call to `ft_sdk::auth_provider::login()` is required.
- added `ft_sdk::data::binary()` and `ft_sdk::data::download()`

## 28th May 2024

### ft-sdk: `0.1.3`

- updated `ft-sys` to `0.1.3`
- bring back `ft_sdk::auth::user_data_by_query`, used by few things behind feature flag

## ft-sys: `0.1.3`

- removed `auth-providers`, `sqlite-default` and `postgres-default` features and `ft_sys::Connection`

### ft-sdk: `0.1.2`

- updated `ft-sys` to `0.1.2`
- added migration framework
- added support for `fastn_user`, `fastn_session`, `fastn_email_queue` tables
- added auth and auth provider framework
- removed `ft_sdk::{CookieExt, Query, QueryExt, JsonBody, JsonBodyExt}`
- removed `ft_sdk::In`
- removed `ft_sdk::Layout` framework
- added `ft_sdk::send_email`
- added `ft_sdk::server_error!()`, `ft_fdk::not_found!()`, `ft_sdk::unauthorised!()`
- added from_request extractors for `cookie`, `form`, `headers`, `hidden`, `host`,
  `path`, `optional`, `mountpoint`, `query`, `required`
- added `ft_sdk::{processor, form, data}` for handling http requests
- added `ft-derive`, and re-exported `ft_sdk::{form, processor, wrapped_processor, data, migration}!()`
- added `ft_sdk::dbg_query`
- added `ft_sdk::Rng` to generate random numbers

### ft-derive: `0.1.0`

Initial release with these macros: `{form, processor, wrapped_processor, data, migration}!()`. Should
be used via `ft-sdk`.

### ft-sys: `0.1.2`

- upgraded `ft-sys-shared` to `0.1.2`
- re-organised code using feature flags
- added sqlite backend
- renamed ft_sys::diesel to ft_sys::diesel_pg
- removed `ft_sys::env::ud`, use `ft_sdk::auth::ud` instead
- `ft_sys::println!` works in non wasm also
- removed non-working `ft_sys::http::{get,post}` and added `ft_sys::http::send`
- created `ft_sys::{Connection, ConnectionError}`

### ft-sys-shared: `0.1.2`

No change, this is good enough for wider use so bumping to `0.1.2`.

## 21st May 2024

### ft-sys-shared: `0.1.1-alpha.4`

- added `SESSION_KEY`

## 18th May 2024

### ft-sys-shared: `0.1.1-alpha.3`

- changed `UserData.username` to `UserData.identity`
- added `DatabaseErrorKind`
- changed `DbError::DatabaseError` (replaced `code` with `kind`)

## 29th Apr 2024

### ft-sys-shared: `0.1.1-alpha.2`

- Added `impl rusqlite::ToSql for SqliteRawValue`

## 27th Apr 2024

### ft-sys-shared: `0.1.1-alpha.1`

- Added sqlite related types.

## 22nd Mar 2024

### ft-sdk `0.1.1`

- Updated: ft-sys `0.1.0 -> 0.1.1`

### ft-sys `0.1.1`

- Added: `ft_sys::env::random() -> f64` to generate a random number.

## 19th Mar 2024

### ft-sdk `0.1.0`

- Initial Release

### ft-sys `0.1.0`

- Initial Release

### ft-sys-shared `0.1.0`

- Initial Release
