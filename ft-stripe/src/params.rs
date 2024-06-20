// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
use serde::{Deserialize, Serialize};

/// A deleted object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deleted<T> {
    /// Unique identifier for the object.
    pub id: T,
    /// Always true for a deleted object.
    pub deleted: bool,
}

/// The `Expand` struct is used to serialize `expand` arguments in retrieve and list apis.
#[doc(hidden)]
#[derive(Serialize)]
pub struct Expand<'a> {
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl Expand<'_> {
    pub(crate) fn is_empty(expand: &[&str]) -> bool {
        expand.is_empty()
    }
}