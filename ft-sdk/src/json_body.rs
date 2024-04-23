pub struct JsonBody {
    pub body: serde_json::Map<String, serde_json::Value>,
}

impl JsonBody {
    pub fn field<T: serde::de::DeserializeOwned>(
        &self,
        field: &str,
    ) -> serde_json::Result<Option<T>> {
        match self.body.get(field) {
            Some(v) => Ok(serde_json::from_value(v.clone())?),
            None => Ok(None),
        }
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_value(serde_json::Value::Object(self.body.clone()))
    }
}

pub trait JsonBodyExt {
    fn json_body(&self) -> serde_json::Result<JsonBody>;
}

impl JsonBodyExt for http::Request<bytes::Bytes> {
    fn json_body(&self) -> serde_json::Result<JsonBody> {
        // TODO: check if content type is application/json
        Ok(JsonBody {
            body: serde_json::from_slice(self.body())?,
        })
    }
}
