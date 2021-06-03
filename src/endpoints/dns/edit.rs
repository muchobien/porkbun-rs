use super::{fill_body_with_record, DnsContent};
use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use serde_json::{Map, Value};
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct EditDns<'a> {
    #[builder(setter(into))]
    record: DnsContent,
    #[builder(setter(into))]
    domain: Cow<'a, str>,
    #[builder(setter(into))]
    id: Cow<'a, str>,
    #[builder(default)]
    ttl: Option<u32>,
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
}

impl<'a> EditDns<'a> {
    pub fn builder() -> EditDnsBuilder<'a> {
        EditDnsBuilder::default()
    }
}

impl<'a> Endpoint for EditDns<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("dns/edit/{}/{}", self.domain, self.id).into()
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
        endpoints::{DnsContent, EditDns},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn record_is_necessary() {
        let err = EditDns::builder().build().unwrap_err();
        assert_eq!("`record` must be initialized", err.to_string())
    }

    #[test]
    fn domain_is_necessary() {
        let err = EditDns::builder()
            .record(DnsContent::Cname {
                content: "".to_string(),
            })
            .build()
            .unwrap_err();
        assert_eq!("`domain` must be initialized", err.to_string())
    }

    #[test]
    fn id_is_necessary() {
        let err = EditDns::builder()
            .domain("")
            .record(DnsContent::Cname {
                content: "".to_string(),
            })
            .build()
            .unwrap_err();
        assert_eq!("`id` must be initialized", err.to_string())
    }

    #[test]
    fn domain_record_and_id_are_sufficient() {
        EditDns::builder()
            .id("1234")
            .domain("example.com")
            .record(DnsContent::Cname {
                content: "".to_string(),
            })
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("dns/edit/example.com/1234")
            .content_type("application/json")
            .body_json(&json!({
                "name": "*",
                "ttl": "600",
                "prio": "600",
                "type": "MX",
                "content": "cnameCname",
            }))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditDns::builder()
            .id("1234")
            .domain("example.com")
            .name("*")
            .record(DnsContent::Mx {
                priority: 600,
                content: "cnameCname".to_string(),
            })
            .ttl(600)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
