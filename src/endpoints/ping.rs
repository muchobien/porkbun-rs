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

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{self, Query},
        endpoints::Ping,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn empty_is_sufficient() {
        Ping::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("ping")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Ping::builder().build().unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
