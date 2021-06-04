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
        let url = client.rest_endpoint(&self.endpoint.endpoint())?;
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
        let url = client.rest_endpoint(&self.endpoint.endpoint())?;
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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use http::{Method, StatusCode};
    use serde_json::json;

    use crate::api::{self, ApiError, AsyncQuery, Endpoint, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }
    }

    #[derive(Debug)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_porkbun_non_json_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        api::ignore(Dummy).query(&client).unwrap()
    }

    #[tokio::test]
    async fn test_porkbun_non_json_response_async() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        api::ignore(Dummy).query_async(&client).await.unwrap()
    }

    #[test]
    fn test_porkbun_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let err = api::ignore(Dummy).query(&client).unwrap_err();
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
                "message": "dummy error message",
                "status": "ERROR",
            }),
        );

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::PorkBun { message, status } = err {
            assert_eq!(message, "dummy error message");
            assert_eq!(status, "ERROR")
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

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::PorkBunUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }
}
