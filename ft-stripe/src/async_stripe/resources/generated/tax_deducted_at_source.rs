// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::TaxDeductedAtSourceId;
use crate::params::{Object, Timestamp};

/// The resource representing a Stripe "TaxDeductedAtSource".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: TaxDeductedAtSourceId,

    /// The end of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: Timestamp,

    /// The start of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: Timestamp,

    /// The TAN that was supplied to Stripe when TDS was assessed.
    pub tax_deduction_account_number: String,
}

impl Object for TaxDeductedAtSource {
    type Id = TaxDeductedAtSourceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_deducted_at_source"
    }
}
