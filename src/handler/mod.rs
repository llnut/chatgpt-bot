use crate::error::ServerError;
use crate::model::ServerRequest;
use axum::response::IntoResponse;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
    TypedHeader,
};
use headers::UserAgent;
use tracing::debug;

pub mod kp;

pub async fn msg_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> Response {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    debug!("`{}` connected.", user_agent);

    ws.on_upgrade(msg_handle_socket)
}

pub async fn msg_handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(t) => {
                debug!("{:?}", t);
                //break;
                //if socket
                //    .send(Message::Text(format!("You said: {t}")))
                //    .await
                //    .is_err()
                //{
                //    break;
                //}
            }
            Message::Binary(d) => {
                debug!(">>> sent {} bytes: {:?}", d.len(), d);
            }
            Message::Close(c) => {
                if let Some(cf) = c {
                    debug!(
                        ">>> sent close with code {} and reason `{}`",
                        cf.code, cf.reason
                    );
                } else {
                    debug!(">>> somehow sent close message without CloseFrame");
                }
                break;
            }

            Message::Pong(v) => {
                debug!(">>> sent pong with {:?}", v);
            }
            // You should never need to manually handle Message::Ping, as axum's websocket library
            // will do so for you automagically by replying with Pong and copying the v according to
            // spec. But if you need the contents of the pings you can see them here.
            Message::Ping(v) => {
                debug!(">>> sent ping with {:?}", v);
            }
        }
    }
}

//use axum::debug_handler;
//#[debug_handler]
pub async fn http_msg_handler(request: ServerRequest) -> Result<impl IntoResponse, ServerError> {
    debug!("http api called: {:?}", request);
    match request {
        ServerRequest::KPRequest(request) => Ok(kp::handle_msg(request).await?),
        _ => panic!("unexpected request type"),
    }
}
