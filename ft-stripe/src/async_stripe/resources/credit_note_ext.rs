// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use crate::client::{Client, Response};
use crate::ids::CreditNoteId;
use crate::resources::CreditNote;

impl CreditNote {
    /// Marks a credit note as void.
    ///
    /// You can only void a credit note if the associated invoice is open.
    pub fn void(client: &Client, id: &CreditNoteId) -> Response<CreditNote> {
        client.post(&format!("/credit_notes/{}/void", id))
    }
}
