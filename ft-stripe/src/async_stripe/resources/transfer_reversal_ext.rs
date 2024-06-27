// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use serde::Deserialize;
use serde::Serialize;

use crate::client::{Client, Response};
use crate::params::Metadata;
use crate::resources::TransferReversal;
use crate::TransferId;

/// The set of parameters that can be used when doing transfer reversal.
///
/// For more details see <https://stripe.com/docs/api/transfer_reversals/create>.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTransferReversal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
}

impl TransferReversal {
    /// Create Transfer Reversal
    ///
    /// For more details see <https://stripe.com/docs/api/transfer_reversals/create>.
    pub fn create(
        client: &Client,
        transfer: &TransferId,
        params: CreateTransferReversal,
    ) -> Response<TransferReversal> {
        client.post_form(&format!("/transfers/{}/reversals", transfer), params)
    }
}
