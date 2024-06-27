// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `Token`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Account,
    BankAccount,
    Card,
    Pii,
}

impl TokenType {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenType::Account => "account",
            TokenType::BankAccount => "bank_account",
            TokenType::Card => "card",
            TokenType::Pii => "pii",
        }
    }
}

impl AsRef<str> for TokenType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for TokenType {
    fn default() -> Self {
        Self::Account
    }
}
