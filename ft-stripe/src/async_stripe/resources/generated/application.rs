// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::ApplicationId;
use crate::params::Object;

/// The resource representing a Stripe "Application".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: ApplicationId,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The name of the application.
    pub name: Option<String>,
}

impl Object for Application {
    type Id = ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "application"
    }
}
