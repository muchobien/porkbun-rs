use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

#[derive(Debug, Builder)]
pub struct DeleteDns<'a> {
    #[builder(setter(into))]
    id: Cow<'a, str>,
    #[builder(setter(into))]
    domain: Cow<'a, str>,
}

impl<'a> DeleteDns<'a> {
    pub fn builder() -> DeleteDnsBuilder<'a> {
        DeleteDnsBuilder::default()
    }
}

impl<'a> Endpoint for DeleteDns<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("dns/delete/{}/{}", self.domain, self.id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{self, Query},
        endpoints::DeleteDns,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn id_is_necessary() {
        let err = DeleteDns::builder().build().unwrap_err();
        assert_eq!("`id` must be initialized", err.to_string())
    }

    #[test]
    fn domain_is_necessary() {
        let err = DeleteDns::builder().id("example.com").build().unwrap_err();
        assert_eq!("`domain` must be initialized", err.to_string())
    }

    #[test]
    fn domain_and_id_are_sufficient() {
        DeleteDns::builder()
            .id("1234")
            .domain("example.com")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("dns/delete/example.com/1234")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteDns::builder()
            .id("1234")
            .domain("example.com")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
