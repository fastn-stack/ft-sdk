// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{FinancialConnectionsAccountOwnershipId};
use crate::params::{List, Object, Timestamp};
use crate::resources::{FinancialConnectionsAccountOwner};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BankConnectionsResourceOwnership".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialConnectionsAccountOwnership {
    /// Unique identifier for the object.
    pub id: FinancialConnectionsAccountOwnershipId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// A paginated list of owners for this account.
    pub owners: List<FinancialConnectionsAccountOwner>,
}

impl Object for FinancialConnectionsAccountOwnership {
    type Id = FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "financial_connections.account_ownership"
    }
}
