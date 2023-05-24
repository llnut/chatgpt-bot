#![allow(dead_code)]

use crate::config::CONFIG;

mod config;
mod error;
mod handler;
mod model;
mod openai;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // enable thread id to be emitted
        .with_thread_ids(CONFIG.log.with_thread_ids)
        // enabled thread name to be emitted
        .with_thread_names(CONFIG.log.with_thread_names)
        .with_env_filter(format!(
            "{},tower={},tower_http={}",
            &CONFIG.log.level, &CONFIG.log.level, &CONFIG.log.level,
        ))
        .init();

    let addr = CONFIG.ip.clone() + ":" + &CONFIG.http_port.to_string();
    println!("listening on {addr}");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(router::app().into_make_service())
        .await
        .unwrap();
}
