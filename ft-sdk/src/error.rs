#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SpecialError {
    #[error("single error {0}: {1}")]
    Single(String, String),
    #[error("multi error {0:?}")]
    Multi(ft_sdk::FormError),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorised: {0}")]
    Unauthorised(String),
}

pub fn single_error<K: AsRef<str>, E: AsRef<str>>(k: K, e: E) -> SpecialError {
    SpecialError::Single(k.as_ref().to_string(), e.as_ref().to_string())
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
    if let Some(field_error) = e.downcast_ref::<SpecialError>() {
        ft_sdk::println!("special error: {field_error}");
        return match field_error {
            SpecialError::Single(k, se) => {
                je(ft_sdk::http::json(serde_json::json!({"errors": {k: se}})))
            }
            SpecialError::Multi(me) => je(ft_sdk::http::json(serde_json::json!({"errors": me}))),
            SpecialError::NotFound(msg) => http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(format!("page not found: {msg}\n").into())
                .unwrap(),
            SpecialError::Unauthorised(msg) => http::Response::builder()
                .status(http::StatusCode::UNAUTHORIZED)
                .body(format!("unauthorised: {msg}\n").into())
                .unwrap(),
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
        // in this example .chain() can not be used as .downcast_ref() on iter returned
        // by chain requires std::error::Error, and http::StatusCode does not implement
        // std::error::Error trait.
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
        Err(EFirst::Yo).context(Status::SeeOther)
    }

    #[test]
    fn t2() {
        let e = outer2().unwrap_err();
        println!("status1: {:?}", e.downcast_ref::<Status>());
        for cause in e.chain() {
            println!("status: {:?}", cause.downcast_ref::<Status>());
        }

        // In this example, the code compiles, but it only finds the status attached
        // by the first function that converts non anyhow::Error to anyhow::Error using
        // the .context(), as shown in first2(). Status attached by out2 and outer2 are
        // simply lost.
        assert!(true)
    }

    fn outer3() -> Result<(), anyhow::Error> {
        Ok(do_something()?)
    }

    #[derive(thiserror::Error, Debug)]
    enum DoSomethingError {
        #[error("get user error {0}")]
        // [from] important
        GetUser(#[from] GetUserError),
    }

    fn do_something() -> Result<(), DoSomethingError> {
        get_user()?;
        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    enum GetUserError {
        #[error("unauthorised {0}")]
        // note that [from] is important here, else error is not added to error chain
        Unauthorised(#[from] super::SpecialError),
    }

    fn get_user() -> Result<i32, GetUserError> {
        Err(GetUserError::Unauthorised(
            super::SpecialError::Unauthorised("yo".to_string()),
        ))
    }

    #[test]
    fn t3() {
        let e = outer3().unwrap_err();

        assert_eq!(e.downcast_ref::<super::SpecialError>(), None);
        let mut iter = e.chain();
        assert_eq!(
            iter.next().unwrap().downcast_ref::<super::SpecialError>(),
            None
        );
        assert_eq!(
            iter.next().unwrap().downcast_ref::<super::SpecialError>(),
            None
        );
        assert_eq!(
            iter.next().unwrap().downcast_ref::<super::SpecialError>(),
            Some(super::SpecialError::Unauthorised("yo".to_string())).as_ref()
        );
        assert!(iter.next().is_none());

        // This example works, but end-user has to make sure std::error::Error traits
        // source() works correctly (here we have used `[from]` to ensure that)
    }
}
