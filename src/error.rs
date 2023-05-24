use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Error as AxumError, Json,
};
use config::ConfigError;
#[cfg(not(feature = "reqwest-client"))]
use curl::Error as CurlError;
#[cfg(feature = "reqwest-client")]
use reqwest::Error as ReqwestError;
use serde_json::json;
use serde_json::Error as SerdeJsonError;
use std::ffi::NulError;
use std::io::Error as IOError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::time::SystemTimeError;
use thiserror::Error;
use tokio::sync::mpsc::error as MpscError;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ServerError {
    #[error("argument error : {0}")]
    ArgumentError(String),
    #[error("config error : {0}")]
    ConfigError(String),
    #[error("json parse error : {0}")]
    JsonParseError(String),
    #[error("Internal Server Error")]
    ServerError,
    #[error("io error : {0}")]
    IOError(String),
    #[error("{0}")]
    Unknown(String),
}

impl<T> From<MpscError::SendError<T>> for ServerError {
    fn from(err: MpscError::SendError<T>) -> Self {
        ServerError::Unknown(err.to_string())
    }
}

impl<T> From<MpscError::SendTimeoutError<T>> for ServerError {
    fn from(err: MpscError::SendTimeoutError<T>) -> Self {
        ServerError::Unknown(err.to_string())
    }
}

impl From<MpscError::TryRecvError> for ServerError {
    fn from(err: MpscError::TryRecvError) -> Self {
        ServerError::Unknown(err.to_string())
    }
}

impl<T> From<MpscError::TrySendError<T>> for ServerError {
    fn from(err: MpscError::TrySendError<T>) -> Self {
        ServerError::Unknown(err.to_string())
    }
}

impl From<Utf8Error> for ServerError {
    fn from(err: Utf8Error) -> Self {
        ServerError::Unknown(err.to_string())
    }
}

impl From<IOError> for ServerError {
    fn from(err: IOError) -> Self {
        ServerError::IOError(err.to_string())
    }
}

impl From<FromUtf8Error> for ServerError {
    fn from(err: FromUtf8Error) -> Self {
        ServerError::IOError(err.to_string())
    }
}

impl From<SerdeJsonError> for ServerError {
    fn from(_err: SerdeJsonError) -> Self {
        tracing::error!("{}", _err.to_string());
        ServerError::JsonParseError("Invalid parameter.".to_string())
    }
}

impl From<AxumError> for ServerError {
    fn from(_err: AxumError) -> Self {
        tracing::error!("{}", _err.to_string());
        ServerError::ServerError
    }
}

impl From<SystemTimeError> for ServerError {
    fn from(_err: SystemTimeError) -> Self {
        tracing::error!("{}", _err.to_string());
        ServerError::ServerError
    }
}

impl From<ConfigError> for ServerError {
    fn from(err: ConfigError) -> Self {
        ServerError::ConfigError(err.to_string())
    }
}

impl From<NulError> for ServerError {
    fn from(_err: NulError) -> Self {
        tracing::error!("{}", _err.to_string());
        ServerError::ServerError
    }
}

#[cfg(feature = "reqwest-client")]
impl From<ReqwestError> for ServerError {
    fn from(_err: ReqwestError) -> Self {
        tracing::error!("{}", _err.to_string());
        ServerError::ServerError
    }
}

#[cfg(not(feature = "reqwest-client"))]
impl From<CurlError> for ServerError {
    fn from(_err: CurlError) -> Self {
        tracing::error!("{}", _err.to_string());
        ServerError::ServerError
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = match self {
            ServerError::ArgumentError(_) => StatusCode::BAD_REQUEST,
            ServerError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::JsonParseError(_) => StatusCode::BAD_REQUEST,
            ServerError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}
