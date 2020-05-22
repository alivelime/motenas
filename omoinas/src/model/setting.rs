use std::collections::HashMap;
use std::default::Default;
use std::env;

use chrono::{FixedOffset, Utc};
use rusoto_dynamodb::{AttributeValue, GetItemInput, PutItemInput};

fn table_setting() -> String {
    return format!("{}_{}", env::var("ENV").unwrap(), "setting");
}
fn key_cotoha_token() -> String {
    return String::from("cotoha_token");
}

pub struct CotohaToken {
    name: String,
    date: String,
    token: String,
}

impl CotohaToken {
    pub fn new(token: &String) -> CotohaToken {
        return CotohaToken {
            name: key_cotoha_token(),
            date: Utc::now()
                .with_timezone(&FixedOffset::east(9 * 3600))
                .format("%Y%m%d")
                .to_string(),
            token: token.clone(),
        };
    }
    pub fn from(item: HashMap<String, AttributeValue>) -> Option<CotohaToken> {
        if item.get("date").is_some() && item["date"].s.is_some() {
            return Some(CotohaToken {
                name: key_cotoha_token(),
                date: item["date"].s.as_ref().unwrap().to_string(),
                token: item["token"].s.as_ref().unwrap().to_string(),
            });
        }
        return None;
    }

    pub fn token(&self) -> String {
        return self.token.clone();
    }
    pub fn date(&self) -> String {
        return self.date.clone();
    }

    pub fn get_item() -> GetItemInput {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert(
            String::from("name"),
            AttributeValue {
                s: Some(key_cotoha_token()),
                ..Default::default()
            },
        );
        return GetItemInput {
            key: key,
            table_name: table_setting(),
            ..Default::default()
        };
    }
    pub fn put_item(&self) -> PutItemInput {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        item.insert(
            String::from("name"),
            AttributeValue {
                s: Some(key_cotoha_token()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("date"),
            AttributeValue {
                s: Some(self.date.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("token"),
            AttributeValue {
                s: Some(self.token.clone()),
                ..Default::default()
            },
        );
        return PutItemInput {
            item: item,
            table_name: table_setting(),
            ..Default::default()
        };
    }
}
