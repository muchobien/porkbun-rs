use crate::{api, auth::Auth};
use async_trait::async_trait;
use bytes::Bytes;
use http::Response as HttpResponse;
use log::{debug, error};
use reqwest::{blocking::Client, Client as AsyncClient};
use serde_json::{Map, Value};
use std::{
    convert::TryInto,
    fmt::{self, Debug},
};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum PorkbunError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
}

type PorkbunResult<T> = Result<T, PorkbunError>;

#[derive(Debug, Error)]
pub enum RestError {
    #[error("communication with porkbun: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

/// A representation of the Porkbun API for a single user.
///
/// Separate users should use separate instances of this.
#[derive(Clone)]
pub struct Porkbun {
    /// The client to use for API calls.
    client: Client,
    /// The base URL to use for API calls.
    url: Url,
    /// The authentication information to use when communicating with PorkBun.
    auth: Auth,
}

impl Porkbun {
    pub fn new(auth: Auth) -> PorkbunResult<Self> {
        let client = Client::new();
        let url = Url::parse("https://porkbun.com/api/json/v3/")?;

        Ok(Self { client, url, auth })
    }
}

impl Debug for Porkbun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Porkbun").field("url", &self.url).finish()
    }
}

impl api::Client for Porkbun {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "porkbun", "REST api call {}", endpoint);
        Ok(self.url.join(endpoint)?)
    }

    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(api::ApiError::client)
    }

    fn auth(&self) -> Map<String, Value> {
        let mut credentials = Map::default();
        credentials.insert("apikey".into(), self.auth.apikey.to_string().into());
        credentials.insert(
            "secretapikey".into(),
            self.auth.secretapikey.to_string().into(),
        );

        credentials
    }
}

#[derive(Clone)]
pub struct AsyncPorkbun {
    /// The client to use for API calls.
    client: reqwest::Client,
    /// The base URL to use for API calls.
    url: Url,
    /// The authentication information to use when communicating with PorkBun.
    auth: Auth,
}

impl Debug for AsyncPorkbun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncPorkbun")
            .field("url", &self.url)
            .finish()
    }
}

#[async_trait]
impl api::AsyncClient for AsyncPorkbun {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        debug!(target: "porkbun", "REST api call {}", endpoint);
        Ok(self.url.join(endpoint)?)
    }

    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };

        call().map_err(api::ApiError::client).await
    }

    fn auth(&self) -> Map<String, Value> {
        let mut credentials = Map::default();
        credentials.insert("apikey".into(), self.auth.apikey.to_string().into());
        credentials.insert(
            "secretapikey".into(),
            self.auth.secretapikey.to_string().into(),
        );

        credentials
    }
}

impl AsyncPorkbun {
    pub fn new(auth: Auth) -> PorkbunResult<Self> {
        let client = AsyncClient::new();
        let url = Url::parse("https://porkbun.com/api/json/v3/")?;

        Ok(Self { client, url, auth })
    }
}
