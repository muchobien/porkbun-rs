use super::error::ApiError;
use async_trait::async_trait;
use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use serde_json::{Map, Value};
use std::error::Error;
use url::Url;

/// A trait representing an asynchronous client which can communicate with a Porkbun instance.
#[async_trait]
pub trait AsyncClient {
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;

    fn auth(&self) -> Map<String, Value>;
}

/// A trait representing a client which can communicate with a Porkbun instance.
pub trait Client {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    /// Send a REST query.
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;

    fn auth(&self) -> Map<String, Value>;
}
