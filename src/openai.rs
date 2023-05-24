use crate::config::CONFIG;
use crate::error::ServerError;
use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashMap;
use std::sync::RwLock;

static REPLY_CACHE: Lazy<RwLock<HashMap<String, Vec<String>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn completion(question: &str) -> Result<Vec<String>, ServerError> {
    if let Some(v) = REPLY_CACHE.read().unwrap().get(question) {
        if v.len() >= CONFIG.reply.cache_per_question {
            let index = {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..v.len())
            };
            return Ok(vec![v[index].to_string()]);
        }
    }

    let mut client = OpenAIClient::new();
    let response = client.query(question.to_string()).await?;

    Ok(response
        .choices
        .into_iter()
        .map(|s| {
            let mut reply = s.content();
            for v in CONFIG.reply.blacklist.iter() {
                if reply.contains(v) {
                    return String::new();
                }
            }
            for v in CONFIG.reply.replace.iter() {
                let index = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..v.1.len())
                };
                reply = reply.replace(v.0, &v.1[index]);
            }

            if let Some(idx) = reply.find("\n\n") {
                if idx < 100 {
                    let reply = reply.split_at(idx + 2).1.replace("\n\n", "\r");
                    println!("reply: {:?}, {:?}", reply, reply.as_bytes());
                    if REPLY_CACHE.read().unwrap().contains_key(question) {
                        REPLY_CACHE
                            .write()
                            .unwrap()
                            .get_mut(question)
                            .unwrap()
                            .push(reply.clone());
                    } else {
                        REPLY_CACHE
                            .write()
                            .unwrap()
                            .insert(question.to_string(), vec![reply.clone()]);
                    }

                    return reply;
                }
            }
            reply
        })
        .collect::<Vec<String>>())
}

use crate::model::openai::{answers::Answers, question::Question};

#[derive(Debug)]
pub struct OpenAIClient {
    #[cfg(feature = "reqwest-client")]
    client: reqwest::Client,
    #[cfg(not(feature = "reqwest-client"))]
    client: curl::easy::Easy,
    api_domain: String,
}

#[cfg(feature = "reqwest-client")]
impl OpenAIClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        Self {
            client,
            api_domain: CONFIG.openai.api_domain.clone(),
        }
    }

    pub async fn query(&self, question: String) -> Result<Answers, ServerError> {
        let question = Question::new_with_default().set_content(question);
        let endpoint = format!("{}/v1/chat/completions", self.api_domain);
        let response = self
            .client
            .post(&endpoint)
            .bearer_auth(&CONFIG.openai.api_key)
            .json(&question)
            .send()
            .await
            .expect("Failed to send request");
        let ans = response
            .json::<Answers>()
            .await
            .expect("Failed to parse response");
        Ok(ans)
    }
}

#[cfg(not(feature = "reqwest-client"))]
impl OpenAIClient {
    pub fn new() -> Self {
        let client = curl::easy::Easy::new();
        Self {
            client,
            api_domain: CONFIG.openai.api_domain.clone(),
        }
    }

    pub async fn query(&mut self, question: String) -> Result<Answers, ServerError> {
        let client = &mut self.client;

        let question = Question::new_with_default().set_content(question);
        let endpoint = format!("{}/v1/chat/completions", self.api_domain);

        let mut response = String::new();
        let json = serde_json::to_string(&question)?;
        let mut headers = curl::easy::List::new();
        headers.append(&format!("Authorization: Bearer {}", &CONFIG.openai.api_key))?;
        headers.append("Content-Type: application/json")?;

        client.url(&endpoint)?;
        client.post(true)?;
        client.http_headers(headers)?;
        client.post_fields_copy(json.as_bytes())?;
        {
            let mut transfer = client.transfer();
            transfer.write_function(|data| {
                response.push_str(&String::from_utf8_lossy(data));
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        let ans = serde_json::from_str::<Answers>(&response).expect("Failed to parse response");
        Ok(ans)
    }
}
