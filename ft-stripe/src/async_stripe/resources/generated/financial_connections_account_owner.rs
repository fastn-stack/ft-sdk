// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{FinancialConnectionsAccountOwnerId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BankConnectionsResourceOwner".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialConnectionsAccountOwner {
    /// Unique identifier for the object.
    pub id: FinancialConnectionsAccountOwnerId,

    /// The email address of the owner.
    pub email: Option<String>,

    /// The full name of the owner.
    pub name: String,

    /// The ownership object that this owner belongs to.
    pub ownership: String,

    /// The raw phone number of the owner.
    pub phone: Option<String>,

    /// The raw physical address of the owner.
    pub raw_address: Option<String>,

    /// The timestamp of the refresh that updated this owner.
    pub refreshed_at: Option<Timestamp>,
}

impl Object for FinancialConnectionsAccountOwner {
    type Id = FinancialConnectionsAccountOwnerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "financial_connections.account_owner"
    }
}
