use log::{debug, error};
use std::collections::HashMap;
use std::default::Default;
use std::env;

use chrono::{FixedOffset, Utc};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};

use crate::cotoha;

fn key_cotoha_token() -> String {
    return String::from("cotoha_token");
}

fn table_setting() -> String {
    return format!("{}_{}", env::var("ENV").unwrap(), "setting");
}

pub fn get_cotoha_token() -> Option<String> {
    let today = Utc::now()
        .with_timezone(&FixedOffset::east(9 * 3600))
        .format("%Y%m%d")
        .to_string();
    let client = DynamoDbClient::new(Region::ApNortheast1);
    let mut key: HashMap<String, AttributeValue> = HashMap::new();
    key.insert(
        String::from("name"),
        AttributeValue {
            s: Some(key_cotoha_token()),
            ..Default::default()
        },
    );
    let get_item = GetItemInput {
        key: key,
        table_name: table_setting(),
        ..Default::default()
    };

    match client.get_item(get_item).sync() {
        Ok(result) => match result.item {
            Some(item) => {
                if item.get("date").is_some() && item["date"].s.is_some() {
                    return item["token"].s.clone();
                }
                debug!("expired cotoha token.");
            }
            None => {
                debug!("no cotoha token.()");
            }
        },
        Err(err) => {
            error!("{}", err);
            return None;
        }
    };

    debug!("get_cotoha_token()");
    match cotoha::api::get_access_token() {
        Ok(t) => {
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
                    s: Some(today),
                    ..Default::default()
                },
            );
            item.insert(
                String::from("token"),
                AttributeValue {
                    s: Some(t.clone()),
                    ..Default::default()
                },
            );
            let put_item = PutItemInput {
                item: item,
                table_name: table_setting(),
                ..Default::default()
            };
            match client.put_item(put_item).sync() {
                Ok(_) => {}
                Err(err) => error!("{}", err),
            }

            return Some(t);
        }
        Err(err) => {
            error!("{}", err);
            return None;
        }
    }
}
