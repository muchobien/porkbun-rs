use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Builder)]
pub struct RetrieveDns<'a> {
    #[builder(setter(into))]
    domain: Cow<'a, str>,
}

impl<'a> RetrieveDns<'a> {
    pub fn builder() -> RetrieveDnsBuilder<'a> {
        RetrieveDnsBuilder::default()
    }
}

impl<'a> Endpoint for RetrieveDns<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("dns/retrieve/{}", self.domain).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{self, Query},
        endpoints::RetrieveDns,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn domain_is_necessary() {
        let err = RetrieveDns::builder().build().unwrap_err();
        assert_eq!("`domain` must be initialized", err.to_string())
    }

    #[test]
    fn domain_is_sufficient() {
        RetrieveDns::builder()
            .domain("example.com")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("dns/retrieve/example.com")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RetrieveDns::builder()
            .domain("example.com")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
