use crate::config::{ServerType, CONFIG};
use crate::error::ServerError;
use crate::model::kp::{KPRequest, Request};
use axum::response::IntoResponse;
#[cfg(target_os = "linux")]
use once_cell::sync::Lazy;
use rand::Rng;
#[cfg(target_os = "linux")]
use ratelimit::Ratelimiter;
use serde::Serialize;
use tracing::debug;

#[cfg(target_os = "linux")]
static RATELIMITER: Lazy<Ratelimiter> = Lazy::new(|| {
    Ratelimiter::new(
        CONFIG.rate_limit.capacity,
        CONFIG.rate_limit.quantum,
        CONFIG.rate_limit.rate,
    )
});

pub async fn handle_msg(request: Request) -> Result<impl IntoResponse, ServerError> {
    #[cfg(target_os = "linux")]
    if RATELIMITER.try_wait().is_err() {
        debug!("ratelimit control");
        return Ok("");
    }

    match request.event_type {
        1100 => text_msg(request).await?,
        n => {
            return Err(ServerError::ArgumentError(format!(
                "unexpedted event_type: {n}",
            )));
        }
    }
    Ok("")
}

pub async fn text_msg(request: Request) -> Result<(), ServerError> {
    let mut content = request.data.content.trim();

    // check prefix
    if let Some(substring) = content.strip_prefix(&CONFIG.reply.prefix) {
        content = substring;
    } else {
        return Ok(());
    }

    if let Some(reply) = CONFIG.reply.text.get(content) {
        let len = reply.len();

        let index = {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..len)
        };

        match CONFIG.server_type {
            ServerType::KPBackend => {
                send_text_msg(
                    &request.data.from_wxid,
                    &request.account_wxid,
                    &reply[index],
                )
                .await?;
            }
            ServerType::CQBackend => {}
        }
    } else if CONFIG.openai.open {
        for v in crate::openai::completion(content).await?.iter() {
            send_text_msg(&request.data.from_wxid, &request.account_wxid, v).await?;
        }
    }
    Ok(())
}

async fn send_text_msg(from_wx_id: &str, robot_wx_id: &str, msg: &str) -> Result<(), ServerError> {
    let json = KPRequest {
        api: "K10033",
        fromWxid: from_wx_id,
        robotWxid: robot_wx_id,
        msg,
    };

    send_to_kp(json).await
}

#[cfg(feature = "reqwest-client")]
async fn send_to_kp<T>(json: T) -> Result<(), ServerError>
where
    T: Serialize,
{
    reqwest::Client::new()
        .post(
            "http://".to_string()
                + &CONFIG.handler.kp.ip.to_string()
                + ":"
                + &CONFIG.handler.kp.port.to_string()
                + "/KP",
        )
        .json(&json)
        .send()
        .await?;
    Ok(())
}

#[cfg(not(feature = "reqwest-client"))]
async fn send_to_kp<T>(json: T) -> Result<(), ServerError>
where
    T: Serialize,
{
    let mut client = curl::easy::Easy::new();

    let endpoint = format!(
        "http://{}:{}/KP",
        &CONFIG.handler.kp.ip, &CONFIG.handler.kp.port
    );

    let mut output = String::new();
    let json = serde_json::to_string(&json)?;
    let mut headers = curl::easy::List::new();
    headers.append("Content-Type: application/json")?;

    client.url(&endpoint)?;
    client.post(true)?;
    client.http_headers(headers)?;
    client.post_fields_copy(json.as_bytes())?;
    {
        let mut transfer = client.transfer();
        transfer.write_function(|data| {
            output.push_str(&String::from_utf8_lossy(data));
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    debug!("send_to_kp response: {:?}", output);
    Ok(())
}
