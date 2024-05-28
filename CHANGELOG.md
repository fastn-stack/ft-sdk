# ChangeLog

## 28th May 2024

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
