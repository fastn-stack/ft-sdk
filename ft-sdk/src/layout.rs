pub trait Page<L>: serde::Serialize {
    fn page(c: &mut L, conn: &mut ft_sdk::Connection) -> Result<Self, ft_sdk::Error>
    where
        Self: Sized;
}

pub trait Action<L> {
    fn validate(c: &mut L, conn: &mut ft_sdk::Connection) -> Result<Self, ft_sdk::Error>
    where
        Self: Sized;
    fn action(
        &self,
        c: &mut L,
        conn: &mut ft_sdk::Connection,
    ) -> Result<ft_sdk::http::Output, ft_sdk::Error>
    where
        Self: Sized;
}

pub trait Layout {
    fn from_in(in_: ft_sdk::In, conn: &mut ft_sdk::Connection) -> Result<Self, ft_sdk::Error>
    where
        Self: Sized;

    fn page<P>(in_: ft_sdk::In, conn: &mut ft_sdk::Connection) -> ft_sdk::http::Result
    where
        P: Page<Self> + serde::Serialize,
        Self: Sized,
    {
        let mut l = Self::from_in(in_.clone(), conn)?;
        let p = P::page(&mut l, conn)?;
        l.json(p)
    }

    fn action<A>(in_: ft_sdk::In, conn: &mut ft_sdk::Connection) -> ft_sdk::http::Result
    where
        A: Action<Self>,
        Self: Sized,
    {
        let mut l = Self::from_in(in_.clone(), conn)?;
        let a = A::validate(&mut l, conn)?;
        a.action(&mut l, conn)
    }

    fn json<P>(&mut self, page: P) -> ft_sdk::http::Result
    where
        P: Page<Self> + serde::Serialize,
        Self: Sized,
    {
        ft_sdk::http::json(serde_json::json!({
            "page": page,
        }))
    }
}
