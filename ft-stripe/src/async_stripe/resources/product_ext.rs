// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use serde::Serialize;

use crate::params::SearchList;
use crate::{Client, Product, Response};

#[derive(Clone, Debug, Default, Serialize)]
pub struct ProductSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> ProductSearchParams<'a> {
    pub fn new() -> ProductSearchParams<'a> {
        ProductSearchParams {
            query: String::new(),
            limit: None,
            page: None,
            expand: &[],
        }
    }
}

impl Product {
    /// Searches for a product.
    ///
    /// For more details see <https://stripe.com/docs/api/products/search>.
    pub fn search(client: &Client, params: ProductSearchParams) -> Response<SearchList<Product>> {
        client.get_query("/products/search", params)
    }
}
