pub trait Page<L, E>: serde::Serialize
where
    E: std::fmt::Debug + From<ft_sdk::Error>,
{
    fn page(c: &mut L) -> Result<Self, E>
    where
        Self: Sized;
}

pub trait Action<L, E>
where
    E: std::fmt::Debug + From<ft_sdk::Error>,
{
    fn validate(c: &mut L) -> Result<Self, E>
    where
        Self: Sized;
    fn action(&self, c: &mut L) -> Result<ActionOutput, E>
    where
        Self: Sized;
}

pub trait ActionWithLog<L, E>
    where
        E: std::fmt::Debug + From<ft_sdk::Error>,
{
    fn validate(c: &mut L) -> Result<Self, E>
        where
            Self: Sized;
    fn action_with_log(&self, c: &mut L) -> Result<ActionOutput, E>
        where
            Self: Sized;
    fn action(&self, c: &mut L) -> Result<ActionOutput, E>
        where
            Self: Sized;
}


#[derive(Debug)]
pub enum ActionOutput {
    Reload,
    Redirect(String),
    Data(FrontendData),
}

pub type FrontendData = std::collections::HashMap<String, serde_json::Value>;

pub enum RequestType {
    Page,
    Action,
}

pub trait Layout {
    type Error: std::fmt::Debug + From<ft_sdk::Error>;

    fn from_in(in_: ft_sdk::In, ty: RequestType) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn _page<P>(r: http::Request<bytes::Bytes>) -> Result<http::Response<bytes::Bytes>, Self::Error>
    where
        P: Page<Self, Self::Error> + serde::Serialize,
        Self: Sized,
    {
        let in_ = ft_sdk::In::from_request(r)?;
        let mut l = Self::from_in(in_, RequestType::Page)?;
        let p = P::page(&mut l)?;
        let vj = serde_json::to_value(&p).unwrap();
        let oj = l.json(vj)?;
        Ok(ft_sdk::json_response(oj))
    }

    fn page<P>(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes>
    where
        P: Page<Self, Self::Error> + serde::Serialize,
        Self: Sized,
    {
        match Self::_page::<P>(r) {
            Ok(r) => r,
            Err(e) => Self::render_error(e),
        }
    }
    fn action<A>(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes>
    where
        A: Action<Self, Self::Error>,
        Self: Sized,
    {
        match Self::_action::<A>(r) {
            Ok(r) => r,
            Err(e) => Self::render_error(e),
        }
    }

    fn _action<A>(
        r: http::Request<bytes::Bytes>,
    ) -> Result<http::Response<bytes::Bytes>, Self::Error>
    where
        A: Action<Self, Self::Error>,
        Self: Sized,
    {
        let in_ = ft_sdk::In::from_request(r)?;
        let mut l = Self::from_in(in_, RequestType::Action)?;
        let a = A::validate(&mut l)?;
        let o = a.action(&mut l)?;
        Ok(a2r(o))
    }

    fn action_with_log<A>(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes>
        where
            A: ActionWithLog<Self, Self::Error>,
            Self: Sized,
    {
        match Self::_action_with_log::<A>(r) {
            Ok(r) => r,
            Err(e) => Self::render_error(e),
        }
    }

    fn _action_with_log<A>(
        r: http::Request<bytes::Bytes>,
    ) -> Result<http::Response<bytes::Bytes>, Self::Error>
        where
            A: ActionWithLog<Self, Self::Error>,
            Self: Sized,
    {
        let start_time = ft_sys::now();
        let in_ = ft_sdk::In::from_request(r)?;
        let mut l = Self::from_in(in_, RequestType::Action)?;
        let a = A::validate(&mut l)?;
        let o = a.action_with_log(&mut l)?;
        let r = a2r(o);
        l.log_to_event(start_time)?;
        Ok(r)
    }

    fn json(&mut self, o: serde_json::Value) -> Result<serde_json::Value, Self::Error>;
    fn render_error(e: Self::Error) -> http::Response<bytes::Bytes>;
    fn log_to_event(&mut self, start_time: chrono::DateTime<chrono::Utc>) -> Result<(),Self::Error>;
}

fn a2r(r: ActionOutput) -> http::Response<bytes::Bytes> {
    ft_sdk::json_response(match r {
        ActionOutput::Reload => serde_json::json!({"reload": true}),
        ActionOutput::Redirect(redirect) => serde_json::json!({"redirect": redirect }),
        ActionOutput::Data(data) => serde_json::json!({"data": data}),
    })
}
