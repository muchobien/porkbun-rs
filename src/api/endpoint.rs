use super::{
    client::{AsyncClient, Client},
    error::ApiError,
    query::{self, AsyncQuery},
    Query, QueryParams,
};
use async_trait::async_trait;
use http::{header, Method, Request};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::borrow::Cow;

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;

    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the data.
    fn body(&self) -> Map<String, Value> {
        Map::default()
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));

        let mut body = self.body();
        body.append(&mut client.auth());

        let data = serde_json::to_vec(&body)?;
        let req = req.header(header::CONTENT_TYPE, "application/json");

        let rsp = client.rest(req, data)?;
        let status = rsp.status();
        let v = serde_json::from_slice(rsp.body())?;
        if !status.is_success() {
            return Err(ApiError::from_porkbun(v));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));

        let mut body = self.body();
        body.append(&mut client.auth());

        let data = serde_json::to_vec(&body)?;
        let req = req.header(header::CONTENT_TYPE, "application/json");

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();
        let v = serde_json::from_slice(rsp.body())?;
        if !status.is_success() {
            return Err(ApiError::from_porkbun(v));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}
