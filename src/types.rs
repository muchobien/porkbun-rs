use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Status {
    status: String,
    message: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    id: String,
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    content: String,
    ttl: String,
    #[serde(rename = "prio")]
    priority: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PingResponse {
    #[serde(rename = "yourIp")]
    your_ip: String,
    #[serde(flatten)]
    status: Status,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateResponse {
    id: String,
    #[serde(flatten)]
    status: Status,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EditResponse {
    #[serde(flatten)]
    status: Status,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteResponse {
    #[serde(flatten)]
    status: Status,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RetrieveResponse {
    records: Vec<Record>,
    #[serde(flatten)]
    status: Status,
}
