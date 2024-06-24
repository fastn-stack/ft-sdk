// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryTransactionsResourceBalanceImpact".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryTransactionsResourceBalanceImpact {

    /// The change made to funds the user can spend right now.
    pub cash: i64,

    /// The change made to funds that are not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,

    /// The change made to funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}
