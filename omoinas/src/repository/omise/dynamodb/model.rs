use std::collections::HashMap;
use std::default::Default;
use std::env;

use log::error;
use rusoto_dynamodb::{AttributeValue, GetItemInput, PutItemInput};

use crate::model::omise::{Omise, Status};

fn table_name() -> String {
    return format!("{}_{}", env::var("ENV").unwrap(), "omise");
}

impl Omise {
    pub fn from(&mut self, item: HashMap<String, AttributeValue>) -> bool {
        if item.get("namae").is_some() && item["namae"].s.is_some() {
            self.namae = item["namae"].s.as_ref().unwrap().to_string();
        }
        if item.get("hp").is_some() && item["hp"].s.is_some() {
            self.hp = item["hp"].s.as_ref().unwrap().to_string();
        }
        if item.get("yotei").is_some() && item["yotei"].s.is_some() {
            self.yotei = item["yotei"].s.as_ref().unwrap().to_string();
        }
        if item.get("status").is_some() && item["status"].n.is_some() {
            self.status = match item["status"].n.as_ref().unwrap().parse::<u32>().unwrap() {
                0 => Status::Yasumi,
                1 => Status::Hima,
                2 => Status::Bochibochi,
                3 => Status::Isogashi,
                4 => Status::Kashikiri,
                s => {
                    error!(
                        "repository::omise::dynamodb::model::from out of status {}",
                        s
                    );
                    return false;
                }
            };
        }
        if item.get("created_at").is_some() && item["created_at"].s.is_some() {
            self.created_at = item["created_at"].s.as_ref().unwrap().to_string();
        }
        return true;
    }

    pub fn get_item(&self) -> GetItemInput {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert(
            String::from("id"),
            AttributeValue {
                s: Some(self.id.clone()),
                ..Default::default()
            },
        );
        return GetItemInput {
            key: key,
            table_name: table_name(),
            ..Default::default()
        };
    }
    pub fn put_item(&self) -> PutItemInput {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        item.insert(
            String::from("id"),
            AttributeValue {
                s: Some(self.id.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("namae"),
            AttributeValue {
                s: Some(self.namae.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("hp"),
            AttributeValue {
                s: Some(self.hp.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("yotei"),
            AttributeValue {
                s: Some(self.yotei.clone()),
                ..Default::default()
            },
        );
        item.insert(
            String::from("status"),
            AttributeValue {
                n: Some((self.status.clone() as i32).to_string()),
                ..Default::default()
            },
        );
        return PutItemInput {
            item: item,
            table_name: table_name(),
            ..Default::default()
        };
    }
}
