use serde_json::{Map, Value};
use std::net::{Ipv4Addr, Ipv6Addr};

mod create;
mod delete;
mod edit;
mod retrieve;

pub use self::create::*;
pub use self::delete::*;
pub use self::edit::*;
pub use self::retrieve::*;

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

pub(crate) fn fill_body_with_record(body: &mut Map<String, Value>, record: &DnsContent) {
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
