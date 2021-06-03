use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use serde_json::{Map, Value};
use std::{
    borrow::Cow,
    net::{Ipv4Addr, Ipv6Addr},
};

#[derive(Debug, Clone)]
pub enum DnsContent {
    NS { content: String },
    A { content: Ipv4Addr },
    TXT { content: String },
    CAA { content: String },
    TLSA { content: String },
    CNAME { content: String },
    AAAA { content: Ipv6Addr },
    MX { content: String, priority: u16 },
    SRV { content: String, priority: u16 },
}

#[derive(Debug, Builder)]
pub struct CreateDns<'a> {
    #[builder(setter(into))]
    record: DnsContent,
    #[builder(setter(into), default)]
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

fn fill_body_with_record(body: &mut Map<String, Value>, record: &DnsContent) {
    match record {
        DnsContent::NS { content } => {
            body.insert("type".into(), "NS".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::A { content } => {
            body.insert("type".into(), "A".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::TXT { content } => {
            body.insert("type".into(), "TXT".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::CAA { content } => {
            body.insert("type".into(), "CAA".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::TLSA { content } => {
            body.insert("type".into(), "TLSA".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::CNAME { content } => {
            body.insert("type".into(), "CNAME".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::AAAA { content } => {
            body.insert("type".into(), "AAAA".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::MX { content, priority } => {
            body.insert("type".into(), "MX".into());
            body.insert("prio".into(), priority.to_string().into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::SRV { content, priority } => {
            body.insert("type".into(), "SRV".into());
            body.insert("prio".into(), priority.to_string().into());
            body.insert("content".into(), content.to_string().into());
        }
    };
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

#[derive(Debug, Builder)]
pub struct EditDns<'a> {
    #[builder(setter(into))]
    record: DnsContent,
    #[builder(setter(into), default)]
    domain: Cow<'a, str>,
    #[builder(setter(into), default)]
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

#[derive(Debug, Builder)]
pub struct DeleteDns<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
    #[builder(setter(into), default)]
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

#[derive(Debug, Builder)]
pub struct RetrieveDns<'a> {
    #[builder(setter(into), default)]
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
