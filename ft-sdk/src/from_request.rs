pub trait FromRequest: Sized {
    fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error>;
}

pub const PATH: usize = 0;

pub struct Request<const NAME: usize> {
    v: String,
}

impl FromRequest for Request<PATH> {
    fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        Ok(Self {
            v: req.uri().path().to_string(),
        })
    }
}

pub fn foo(Request { v: path }: Request<PATH>, RString { v: path2 }: RString<"path">) {
    println!("{}", path2);
    if path.is_empty() {
        println!("is empty")
    } else {
        println!("{}", path);
    }
}

pub struct RString<const NAME: &'static str> {
    v: String,
}
