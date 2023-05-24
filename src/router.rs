use crate::handler;
use axum::{
    body::Bytes,
    http::{HeaderMap, Request},
    response::Response,
    routing::{get, post},
    Router,
};
use std::time::Duration;
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::Span;

pub fn app() -> Router {
    Router::new()
        .route("/msg-http", post(handler::http_msg_handler))
        .route("/msg", get(handler::msg_handler))
        .route("/msg-post", post(handler::msg_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true))
                .on_request(|_request: &Request<_>, _span: &Span| {
                    tracing::trace!("{:?}\n{:?}\n", _request, _span);
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    tracing::trace!("{:?}\n{:?}\n{:?}\n", _response, _latency, _span);
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    tracing::trace!("{:?}\n{:?}\n{:?}\n", _chunk, _latency, _span);
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        tracing::trace!("{:?}\n{:?}\n{:?}\n", _trailers, _stream_duration, _span);
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::trace!("{:?}\n{:?}\n{:?}\n", _error, _latency, _span);
                    },
                ),
        )
}
