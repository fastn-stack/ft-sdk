// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::ApplicationId;
use crate::params::Object;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Application".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: ApplicationId,

    /// The name of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
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
