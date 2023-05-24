use crate::config::{ServerType, CONFIG};
use axum::{
    async_trait,
    extract::FromRequest,
    http::Request,
    response::{IntoResponse, Response},
    Json, RequestExt,
};
use serde::{Deserialize, Serialize};

pub mod kp;
pub mod openai;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerRequest {
    KPRequest(kp::Request),
    CQRequest(kp::Request),
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ServerRequest
where
    B: Send + 'static,
    S: Send + Sync,
    Json<kp::Request>: FromRequest<(), B>,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        match CONFIG.server_type {
            ServerType::KPBackend => {
                let Json::<kp::Request>(payload) =
                    req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self::KPRequest(payload));
            }
            ServerType::CQBackend => {
                let Json::<kp::Request>(payload) =
                    req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self::KPRequest(payload));
            }
        }
    }
}
