use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub event_type: u32,
    pub event_desc: String,
    pub account_wxid: String,
    pub data: RequestData,
    pub pid: u32,
    pub hwnd: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RequestData {
    pub SvrId: String,
    pub LocalId: u32,
    pub PrefixId: String,
    pub createtime: u64,
    pub msgtype: u8,
    pub subtype: u8,
    pub isSender: u8,
    pub content: String,
    pub from_wxid: String,
    pub from_name: String,
    pub final_from_wxid: String,
    pub final_from_name: String,
    pub final_displayname: String,
    pub to_wxid: String,
    pub to_name: String,
    pub filepath: String,
    pub atuserlist: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub code: u16,
    pub msg: Option<String>,
    pub data: Option<String>,
}

impl Response {
    pub fn success(data: Option<String>) -> Self {
        Self {
            code: 200,
            msg: Some("succeed".to_string()),
            data,
        }
    }

    pub fn fail(code: u16, msg: String) -> Self {
        Self {
            code,
            msg: Some(msg),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct KPRequest<'a> {
    pub api: &'a str,
    pub robotWxid: &'a str,
    pub fromWxid: &'a str,
    pub msg: &'a str,
}
