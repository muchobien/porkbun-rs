use crate::api::{query, ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query};
use async_trait::async_trait;
use http::{header, Request};

/// A query modifier that ignores the data returned from an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ignore<E> {
    endpoint: E,
}

/// Ignore the resulting data from an endpoint.
pub fn ignore<E>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}

impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let mut req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));

        let mut body = self.endpoint.body();
        body.append(&mut client.auth());

        let data = match body.len() {
            0 => vec![],
            _ => {
                req = req.header(header::CONTENT_TYPE, "application/json");
                serde_json::to_vec(&body)?
            }
        };

        let rsp = client.rest(req, data)?;
        if !rsp.status().is_success() {
            let v = serde_json::from_slice(rsp.body())?;
            return Err(ApiError::from_porkbun(v));
        }

        Ok(())
    }
}

#[async_trait]
impl<E, C> AsyncQuery<(), C> for Ignore<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let mut req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));

        let mut body = self.endpoint.body();
        body.append(&mut client.auth());

        let data = match body.len() {
            0 => vec![],
            _ => {
                req = req.header(header::CONTENT_TYPE, "application/json");
                serde_json::to_vec(&body)?
            }
        };

        let rsp = client.rest_async(req, data).await?;
        if !rsp.status().is_success() {
            let v = serde_json::from_slice(rsp.body())?;
            return Err(ApiError::from_porkbun(v));
        }

        Ok(())
    }
}
