pub struct Query {
    args: Vec<(String, String)>,
}

impl Query {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.args
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

pub trait QueryExt {
    fn query(&self) -> Query;
}

impl QueryExt for http::Request<bytes::Bytes> {
    fn query(&self) -> Query {
        let query = self.uri().query().unwrap_or_default();
        let args = match serde_urlencoded::from_str(query) {
            Ok(v) => v,
            Err(_) => {
                ft_sdk::println!("failed to parse query: {}", query);
                vec![]
            }
        };
        Query { args }
    }
}
