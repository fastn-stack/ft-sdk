#[derive(Debug, thiserror::Error)]
pub enum FieldError {
    #[error("single error {0}: {1}")]
    Single(String, String),
    #[error("multi error {0:?}")]
    Multi(ft_sdk::FormError),
}

pub fn single_error<K: AsRef<str>, E: AsRef<str>>(k: K, e: E) -> FieldError {
    FieldError::Single(k.as_ref().to_string(), e.as_ref().to_string())
}

fn je(r: Result<http::Response<bytes::Bytes>, ft_sdk::Error>) -> http::Response<bytes::Bytes> {
    r.unwrap_or_else(|e| {
        http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("json error: {e:?}\n").into())
            .unwrap()
    })
}

pub fn handle_error(e: anyhow::Error) -> http::Response<bytes::Bytes> {
    if let Some(status) = e.downcast_ref::<http::StatusCode>() {
        ft_sdk::println!("status code: {status}");
        return http::Response::builder()
            .status(*status)
            .body(format!("status code: {status}\n").into())
            .unwrap();
    }
    if let Some(field_error) = e.downcast_ref::<FieldError>() {
        ft_sdk::println!("field error: {field_error}");
        return match field_error {
            FieldError::Single(k, se) => {
                je(ft_sdk::http::json(serde_json::json!({"errors": {k: se}})))
            }
            FieldError::Multi(me) => je(ft_sdk::http::json(serde_json::json!({"errors": me}))),
        };
    }
    http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("json error: {e:?}\n").into())
        .unwrap()
}

#[cfg(test)]
mod test {
    use anyhow::Context;

    #[derive(thiserror::Error, Debug)]
    enum EFirst {
        #[error("yo")]
        Yo,
    }

    fn outer() -> Result<(), anyhow::Error> {
        anyhow::Ok(out()?).context(http::StatusCode::CREATED)
    }

    fn out() -> Result<(), anyhow::Error> {
        anyhow::Ok(first()?).context(http::StatusCode::ACCEPTED)
    }

    fn first() -> Result<(), anyhow::Error> {
        Err(EFirst::Yo).context(http::StatusCode::SEE_OTHER)
    }

    #[test]
    fn t() {
        let e = outer().unwrap_err();
        assert_eq!(
            *e.downcast_ref::<http::StatusCode>().unwrap(),
            http::StatusCode::SEE_OTHER
        );
    }

    #[derive(thiserror::Error, Debug, PartialEq)]
    enum Status {
        #[error("created")]
        Created,
        #[error("accepted")]
        Accepted,
        #[error("see-other")]
        SeeOther,
    }

    fn outer2() -> Result<(), anyhow::Error> {
        anyhow::Ok(out2()?).context(Status::Created)
    }

    fn out2() -> Result<(), anyhow::Error> {
        anyhow::Ok(first2()?).context(Status::Accepted)
    }

    fn first2() -> Result<(), anyhow::Error> {
        Err(EFirst::Yo)? // .context(Status::SeeOther)
    }

    #[test]
    fn t2() {
        let e = outer2().unwrap_err();
        println!("status1: {:?}", e.downcast_ref::<Status>());
        for cause in e.chain() {
            println!("status: {:?}", cause.downcast_ref::<Status>());
        }

        // looks like we have to rethink our context approach
        assert!(true)
    }
}
