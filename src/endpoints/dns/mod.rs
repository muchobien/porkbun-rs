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
    Ns { content: String },
    A { content: Ipv4Addr },
    Txt { content: String },
    Caa { content: String },
    Tlsa { content: String },
    Cname { content: String },
    Aaaa { content: Ipv6Addr },
    Mx { content: String, priority: u16 },
    Srv { content: String, priority: u16 },
}

pub(crate) fn fill_body_with_record(body: &mut Map<String, Value>, record: &DnsContent) {
    match record {
        DnsContent::Ns { content } => {
            body.insert("type".into(), "NS".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::A { content } => {
            body.insert("type".into(), "A".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::Txt { content } => {
            body.insert("type".into(), "TXT".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::Caa { content } => {
            body.insert("type".into(), "CAA".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::Tlsa { content } => {
            body.insert("type".into(), "TLSA".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::Cname { content } => {
            body.insert("type".into(), "CNAME".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::Aaaa { content } => {
            body.insert("type".into(), "AAAA".into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::Mx { content, priority } => {
            body.insert("type".into(), "MX".into());
            body.insert("prio".into(), priority.to_string().into());
            body.insert("content".into(), content.to_string().into());
        }
        DnsContent::Srv { content, priority } => {
            body.insert("type".into(), "SRV".into());
            body.insert("prio".into(), priority.to_string().into());
            body.insert("content".into(), content.to_string().into());
        }
    };
}
