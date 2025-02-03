# ChangeLog

## Unreleased

### ft-sdk: unreleased (0.2.1)

- BREAKING: removed `ft_sdk::send_email()`, added `ft_sdk::email::{send, 
  cancel}()`. Not moving to 0.3 as 0.2 was just released an hour ago.
- re-exported `ft_sys_shared::{Email, EmailAddress, EmailHandle}`
- added `ft_sdk::Config` extractor
- bump `ft-sys` to 0.1.7
- bump `ft-sys-shared` to 0.1.5

### ft-sys: unreleased (0.1.6)

- bump `ft-sys-shared` to 0.1.5

## 3rd Feb 2025

### ft-sys-shared: unreleased (0.1.6)

- Downgrade rusqlite to 0.31 as needed by fastn etc

### ft-sys-shared: unreleased (0.1.5)

- added `ft_sys_shared::{Email, EmailAddress, EmailHandle}`

### ft-sdk: `0.2.0`

- BREAKING: renamed `ft_sdk::Mountpoint` -> `ft_sdk::AppUrl<KEY="current-app">`,
  the inner value is now an `Option<String>` instead of `String`.
- Added `ft_sdk::AppUrl::{join, is_set}()`
- Added `ft_sdk::Result<T>` alias to `Result<T, ft_sdk::Error>`.
- Added `ft_sdk::Scheme` extractor.

## 11th Oct 2024

### ft-sdk: `0.1.17`

- bumped `ft-sys` to `0.1.6`

### ft-sys: `0.1.6`

- upgraded `diesel` from `2.1` to `2.2`

## 10th July 2024

### ft-sdk: `0.1.16`

- bumped `ft-sys` to `0.1.5`
- bumped `ft-sys-shared` to `0.1.4`

### ft-sys: `0.1.5`

- bumped `ft-sys-shared` to `0.1.4`

### ft-sys-shared: `0.1.4`

- `impl TryFrom<&SqliteRawValue> for Vec<u8>` accepts `SqliteRawValue::Text()`

## 2nd July 2024

### ft-sdk: `0.1.15`

- fixed `ft_sdk::data::browser_redirect_with_cookie()`, it was not setting
  cookie earlier.

## 28th June 2024

### ft-sdk: `0.1.14`

- Due to mistake in previous release, the ft-sys and ft-sys-shared were not
  upgraded
  to latest versions.

### ft-sdk: `0.1.13`

- Decoupled session store from `auth-provider` feature. `ft_sdk::session` module
  can
  be used to interact with the session store
- Move `ft_sdk::auth::SessionID` to `ft_sdk::SessionID`. The `ft_sdk::session`
  replaces
  `ft_sdk::auth::session`
- add `ft_sdk::SessionData` which can only be constructed through
  `SessionId::data()`
  and is used to interact with the session store.
- `ft_sdk::utils::uuid_v8` function to generate uuids.
- Impl `Display` for `ft_sdk::PlainText`.
- Derive `Clone` for `ft_sdk::Cookie` and `ft_sdk::Host`
- Added `ft_sdk::auth::provider::user_data_by_id` convenience function to get
  user data by id.
- Added `ft_sdk::auth::ProviderData::first_email` to get the first email (
  verified or
  unverified) of the user.
- Removed `ft_sdk::auth::provider::LoginError::SetUserIDError` and added
  `ft_sdk::auth::provider::LoginError::SessionError`
- Added `ft_sdk::data::browser_redirect_with_cookie()`: redirect and set-cookie
  do
  not work well together, this function be used to work around that issue.
- Bumped `ft-sys` to `0.1.4`
- Bumped `ft-sys-shared` to `0.1.3

### ft-sys: `0.1.4`

- Bumped `ft-sys-shared` to `0.1.3`

### ft-sys-shared: `0.1.3`

- Added `ft_sys_shared::TRACKER_KEY` constant

## 14th June 2024

### ft-sdk: `0.1.12`

- fix: fill `UserData::email` with user's verified emails (or unverified emails
  if
  no verified email is present).
- fix: use `fastn_user.identity` column to fill `UserData::identity`.
- make `ft_sdk::auth::user_data_by_query` public.

## 12th June 2024

### ft-sdk: `0.1.11`

- Bug fix: Cookie related bug introduced in previous release.

### ft-sdk: `0.1.10`

- Bug fix: `fastn-sid-1` cookie was found when we try
  `ft_sdk::Cookie<"fastn-sid">`.

## 11th June 2024

### ft-sdk: `0.1.9`

- removed `ft_sdk::processor::redirect()` in favor of
  `ft_sdk::processor::{temporary,permanent}_redirect()`
- the `ft_sdk::processor::*_redirect` helpers return redirect response instead
  of json response

## 6th June 2024

### ft-sdk: `0.1.8`

- added default `String` to `ft_sdk::Query<const KEY: &'static str, T = String>`
  so its easy to
  migrate from `ft-sdk: 0.1.6`.

### ft-sdk: `0.1.7`

- removed migration framework. we are adding migration feature to fastn core
  itself, so no need to
  maintain it here.
- support for optional query parameters in `ft_sdk::Query`.
- ft-derive 0.1.1

### ft-derive: `0.1.1`

- removed migration proc macro

## 3rd June 2024

### ft-sdk: `0.1.6`

- added `#[serde(default)]` to `ft_sdk::auth::ProviderData`
- added `#[derive(Default)` to `ft_sdk::auth::ProviderData`
- added `#[derive(Debug)` to `ft_sdk::auth::{UserId, SessionID, Counter}`

## 31st May 2024

### ft-sdk: `0.1.5`

- `ft_sdk::auth::provider::user_data_by_custom_attribute`: get the user that
  matching the provided
  custom key.
- make `fastn_user::identity` nullable, `fastn_user::data` non-nullable and
  `fastn_session::data`
  non-nullable.
- updated `diesel` requirement to `">=2, <2.2"`, we do not yet work with
  recently released diesel-2.2.

## 30th May 2024

### ft-sdk: `0.1.4`

- added `ft_sdk::auth_provider::update_user()`
- `ft_sdk::auth_provider::create_user()` no longer returns `session id`, instead
  returns the `user id`
  of the created user. also it does not automatically log user in, call to
  `ft_sdk::auth_provider::login()`
  is now required to explicitly log user in.
- added `ft_sdk::data::binary()` and `ft_sdk::data::download()` helpers to
  construct http responses.

## 28th May 2024

### ft-sdk: `0.1.3`

- updated `ft-sys` to `0.1.3`
- bring back `ft_sdk::auth::user_data_by_query`, used by few things behind
  feature flag

## ft-sys: `0.1.3`

- removed `auth-providers`, `sqlite-default` and `postgres-default` features and
  `ft_sys::Connection`

### ft-sdk: `0.1.2`

- updated `ft-sys` to `0.1.2`
- added migration framework
- added support for `fastn_user`, `fastn_session`, `fastn_email_queue` tables
- added auth and auth provider framework
- removed `ft_sdk::{CookieExt, Query, QueryExt, JsonBody, JsonBodyExt}`
- removed `ft_sdk::In`
- removed `ft_sdk::Layout` framework
- added `ft_sdk::send_email`
- added `ft_sdk::server_error!()`, `ft_fdk::not_found!()`,
  `ft_sdk::unauthorised!()`
- added from_request extractors for `cookie`, `form`, `headers`, `hidden`,
  `host`,
  `path`, `optional`, `mountpoint`, `query`, `required`
- added `ft_sdk::{processor, form, data}` for handling http requests
- added `ft-derive`, and re-exported
  `ft_sdk::{form, processor, wrapped_processor, data, migration}!()`
- added `ft_sdk::dbg_query`
- added `ft_sdk::Rng` to generate random numbers

### ft-derive: `0.1.0`

Initial release with these macros:
`{form, processor, wrapped_processor, data, migration}!()`. Should
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
