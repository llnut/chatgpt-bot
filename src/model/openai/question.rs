#![allow(dead_code)]
use super::{Message, Role};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub model: String,
    pub messages: Vec<Message>,
}

impl Question {
    #![allow(clippy::too_many_arguments)]
    pub fn new(model: String, content: String) -> Self {
        Self {
            model,
            messages: vec![Message {
                role: Role::User,
                content,
            }],
        }
    }

    pub fn new_with_default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: Vec::new(),
        }
    }

    pub fn set_content(mut self, content: String) -> Self {
        self.messages.push(Message {
            role: Role::User,
            content,
        });
        self
    }
}
