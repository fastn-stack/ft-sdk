// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::params::Expandable;
use crate::resources::Account;

/// The resource representing a Stripe "ConnectAccountReference".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectAccountReference {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Expandable<Account>>,

    /// Type of the account referenced.
    #[serde(rename = "type")]
    pub type_: ConnectAccountReferenceType,
}

/// An enum representing the possible values of an `ConnectAccountReference`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectAccountReferenceType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl ConnectAccountReferenceType {
    pub fn as_str(self) -> &'static str {
        match self {
            ConnectAccountReferenceType::Account => "account",
            ConnectAccountReferenceType::Self_ => "self",
        }
    }
}

impl AsRef<str> for ConnectAccountReferenceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConnectAccountReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ConnectAccountReferenceType {
    fn default() -> Self {
        Self::Account
    }
}
