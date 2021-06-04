use super::{
    client::{AsyncClient, Client},
    error::ApiError,
    query::{self, AsyncQuery},
    Query,
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
        let url = client.rest_endpoint(&self.endpoint())?;
        let mut req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let mut body = self.body();
        body.append(&mut client.auth());

        let data = match body.len() {
            0 => vec![],
            _ => {
                req = req.header(header::CONTENT_TYPE, "application/json");
                serde_json::to_vec(&body)?
            }
        };

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
        let url = client.rest_endpoint(&self.endpoint())?;
        let mut req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let mut body = self.body();
        body.append(&mut client.auth());

        let data = match body.len() {
            0 => vec![],
            _ => {
                req = req.header(header::CONTENT_TYPE, "application/json");
                serde_json::to_vec(&body)?
            }
        };

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();
        let v = serde_json::from_slice(rsp.body())?;
        if !status.is_success() {
            return Err(ApiError::from_porkbun(v));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}

#[cfg(test)]
mod tests {
    use http::{Method, StatusCode};
    use serde::Deserialize;
    use serde_json::json;
    use std::borrow::Cow;

    use crate::{
        api::{ApiError, AsyncQuery, Query},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    use super::Endpoint;

    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }
    }

    #[derive(Debug, Deserialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_porkbun_empty_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Json { source } = err {
            assert_eq!(
                format!("{}", source),
                "EOF while parsing a value at line 1 column 0",
            );
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_porkbun_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Json { source } = err {
            assert_eq!(
                format!("{}", source),
                "EOF while parsing a value at line 1 column 0",
            );
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_porkbun_error_detection() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "status": "ERROR",
                "message": "All HTTP request must use POST."
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::PorkBun { message, status } = err {
            assert_eq!(message, "All HTTP request must use POST.");
            assert_eq!(status, "ERROR");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_porkbun_error_detection_unknown() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let client = SingleTestClient::new_json(endpoint, &err_obj);

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::PorkBunUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_bad_deserialization() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "not_value": 0,
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::DataType { source, typename } = err {
            assert_eq!(format!("{}", source), "missing field `value`");
            assert_eq!(typename, "porkbun_rs::api::endpoint::tests::DummyResult");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_good_deserialization() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );

        let res: DummyResult = Dummy.query(&client).unwrap();
        assert_eq!(res.value, 0);
    }

    #[tokio::test]
    async fn test_good_deserialization_async() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );

        let res: DummyResult = Dummy.query_async(&client).await.unwrap();
        assert_eq!(res.value, 0);
    }
}
