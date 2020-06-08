use std::collections::HashMap;
use std::default::Default;
use std::env;

use chrono::{FixedOffset, Utc};
use rusoto_dynamodb::{AttributeValue, GetItemInput, PutItemInput};

use crate::model::setting::ParserToken;
use crate::util::dynamodb::*;

fn table_name() -> String {
    return format!("{}_{}", env::var("ENV").unwrap(), "setting");
}
fn key_cotoha_token() -> String {
    return String::from("cotoha_token");
}

impl ParserToken {
    pub fn new(token: &String) -> ParserToken {
        return ParserToken {
            name: key_cotoha_token(),
            date: Utc::now()
                .with_timezone(&FixedOffset::east(9 * 3600))
                .format("%Y%m%d")
                .to_string(),
            token: token.clone(),
        };
    }
    pub fn from(item: HashMap<String, AttributeValue>) -> Option<ParserToken> {
        if item.get("date").is_some() && item["date"].s.is_some() {
            return Some(ParserToken {
                name: key_cotoha_token(),
                date: item["date"].s.as_ref().unwrap().to_string(),
                token: item["token"].s.as_ref().unwrap().to_string(),
            });
        }
        return None;
    }

    pub fn get_item() -> GetItemInput {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key_insert_str(&mut key, key_cotoha_token(), "name");
        return GetItemInput {
            key: key,
            table_name: table_name(),
            ..Default::default()
        };
    }
    pub fn put_item(&self) -> PutItemInput {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        key_insert_str(&mut item, key_cotoha_token(), "name");
        key_insert_str(&mut item, self.date.clone(), "date");
        key_insert_str(&mut item, self.token.clone(), "token");
        return PutItemInput {
            item: item,
            table_name: table_name(),
            ..Default::default()
        };
    }
}
