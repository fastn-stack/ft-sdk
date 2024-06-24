// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum CardBrand {
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Diners Club")]
    DinersClub,
    #[serde(rename = "Discover")]
    Discover,
    #[serde(rename = "JCB")]
    JCB,
    #[serde(rename = "Visa")]
    Visa,
    #[serde(rename = "MasterCard")]
    MasterCard,
    #[serde(rename = "UnionPay")]
    UnionPay,

    /// An unknown card brand.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "Unknown")]
    Unknown,
}

impl std::default::Default for CardBrand {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum CardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "prepaid")]
    Prepaid,

    /// An unknown card type.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::default::Default for CardType {
    fn default() -> Self {
        Self::Unknown
    }
}
