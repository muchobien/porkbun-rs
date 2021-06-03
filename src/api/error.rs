use serde::Deserialize;
use std::{any, error::Error};
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct Status {
    status: String,
    message: String,
}

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    #[error("failed to encode body parameters: {}", source)]
    JsonEncodee {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}

#[derive(Debug, Error)]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    // The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
    /// Body data could not be created.
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },
    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
    /// PorkBun returned an error message.
    #[error("porkbun server error: {} status: {}", message, status)]
    PorkBun {
        /// The error message from porkbun.
        message: String,
        /// The error status from porkbun.
        status: String,
    },
    /// PorkBun returned an HTTP error with JSON we did not recognize.
    #[error("porkbun server error: {:?}", obj)]
    PorkBunUnrecognized {
        /// The full object from PorkBun.
        obj: serde_json::Value,
    },
    /// JSON deserialization from PorkBun failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        ApiError::Client { source }
    }

    pub(crate) fn from_porkbun(value: serde_json::Value) -> Self {
        match serde_json::from_value::<Status>(value.clone()) {
            Ok(status) => ApiError::PorkBun {
                status: status.status,
                message: status.message,
            },
            Err(_) => ApiError::PorkBunUnrecognized { obj: value },
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use thiserror::Error;

    use crate::api::ApiError;

    #[derive(Debug, Error)]
    #[error("my error")]
    enum MyError {}

    #[test]
    fn porkbun_error_message_string() {
        let obj = json!({
            "status": "ERROR",
            "message": "All HTTP request must use POST."
        });

        let err: ApiError<MyError> = ApiError::from_porkbun(obj);
        if let ApiError::PorkBun { message, status } = err {
            assert_eq!(message, "All HTTP request must use POST.");
            assert_eq!(status, "ERROR");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn gitlab_error_message_unrecognized() {
        let err_obj = json!({
            "some_weird_key": "an even weirder value",
        });

        let err: ApiError<MyError> = ApiError::from_porkbun(err_obj.clone());
        if let ApiError::PorkBunUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }
}
