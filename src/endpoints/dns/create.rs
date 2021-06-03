use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use serde_json::{Map, Value};
use std::borrow::Cow;

use super::{fill_body_with_record, DnsContent};

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct CreateDns<'a> {
    #[builder(setter(into))]
    record: DnsContent,
    #[builder(setter(into))]
    domain: Cow<'a, str>,
    #[builder(default)]
    ttl: Option<u32>,
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
}

impl<'a> CreateDns<'a> {
    pub fn builder() -> CreateDnsBuilder<'a> {
        CreateDnsBuilder::default()
    }
}

impl<'a> Endpoint for CreateDns<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("dns/create/{}", self.domain).into()
    }

    fn body(&self) -> Map<String, Value> {
        let mut body = Map::default();
        if let Some(name) = &self.name {
            body.insert("name".into(), name.to_string().into());
        }

        if let Some(ttl) = self.ttl {
            body.insert("ttl".into(), ttl.to_string().into());
        }

        fill_body_with_record(&mut body, &self.record);

        body
    }
}

#[cfg(test)]
mod tests {
    use http::Method;
    use serde_json::json;

    use crate::{
        api::{self, Query},
        endpoints::{CreateDns, DnsContent},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn record_is_necessary() {
        let err = CreateDns::builder().build().unwrap_err();
        assert_eq!("`record` must be initialized", err.to_string())
    }

    #[test]
    fn domain_is_necessary() {
        let err = CreateDns::builder()
            .record(DnsContent::CNAME {
                content: "".to_string(),
            })
            .build()
            .unwrap_err();
        assert_eq!("`domain` must be initialized", err.to_string())
    }

    #[test]
    fn domain_and_record_are_sufficient() {
        CreateDns::builder()
            .domain("example.com")
            .record(DnsContent::CNAME {
                content: "".to_string(),
            })
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("dns/create/example.com")
            .content_type("application/json")
            .body_json(&json!({
                "name": "*",
                "ttl": "600",
                "prio": "600",
                "type": "MX",
                "content": "cname",
            }))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateDns::builder()
            .domain("example.com")
            .record(DnsContent::MX {
                priority: 600,
                content: "cname".to_string(),
            })
            .ttl(600)
            .name("*")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
