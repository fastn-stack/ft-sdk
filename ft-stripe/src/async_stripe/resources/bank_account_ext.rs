// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `BankAccount`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountStatus {
    Errored,
    New,
    Validated,
    VerificationFailed,
    Verified,
}

impl BankAccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BankAccountStatus::Errored => "errored",
            BankAccountStatus::New => "new",
            BankAccountStatus::Validated => "validated",
            BankAccountStatus::VerificationFailed => "verification_failed",
            BankAccountStatus::Verified => "verified",
        }
    }
}

impl AsRef<str> for BankAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
