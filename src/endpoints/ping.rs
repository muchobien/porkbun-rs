use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

#[derive(Debug, Builder)]
pub struct Ping {}

impl Ping {
    pub fn builder() -> PingBuilder {
        PingBuilder::default()
    }
}

impl Endpoint for Ping {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "ping".into()
    }
}
