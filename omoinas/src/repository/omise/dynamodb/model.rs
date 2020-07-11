use std::collections::HashMap;
use std::default::Default;
use std::env;

use rusoto_dynamodb::{AttributeValue, GetItemInput, PutItemInput};

use crate::model::omise::Omise;
use crate::util::dynamodb::*;

fn table_name() -> String {
    return format!("{}_{}", env::var("ENV").unwrap(), "omise");
}

impl Omise {
    pub fn from(&mut self, item: HashMap<String, AttributeValue>) -> bool {
        item_from_str(&item, &mut self.client_id, "client_id");
        item_from_str(&item, &mut self.omise_id, "omise_id");
        item_from_str(&item, &mut self.namae, "namae");
        item_from_json(&item, &mut self.link, "link");
        item_from_str(&item, &mut self.yotei, "yotei");
        item_from_str(&item, &mut self.oshirase, "oshirase");
        item_from_json(&item, &mut self.otokoro, "otokoro");
        item_from_ss(&item, &mut self.omotenashi, "omotenashi");
        item_from_ss(&item, &mut self.oshiharai, "oshiharai");
        item_from_json(&item, &mut self.ima, "ima");
        item_from_str(&item, &mut self.hitokoto, "hitokoto");
        item_from_datetime(&item, &mut self.kefu_kara, "kefu_kara");
        item_from_datetime(&item, &mut self.kefu_made, "kefu_made");
        item_from_ss(&item, &mut self.tanamono, "tanamono");
        item_from_datetime(&item, &mut self.created_at, "created_at");
        item_from_datetime(&item, &mut self.updated_at, "updated_at");
        return true;
    }

    pub fn get_item(&self) -> GetItemInput {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key_insert_str(&mut key, self.client_id.clone(), "client_id");
        key_insert_str(&mut key, self.omise_id.clone(), "omise_id");
        return GetItemInput {
            key: key,
            table_name: table_name(),
            ..Default::default()
        };
    }
    pub fn put_item(&self) -> PutItemInput {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        key_insert_str(&mut item, self.client_id.clone(), "client_id");
        key_insert_str(&mut item, self.omise_id.clone(), "omise_id");
        key_insert_str(&mut item, self.namae.clone(), "namae");
        key_insert_json(&mut item, &self.link, "link");
        key_insert_str(&mut item, self.yotei.clone(), "yotei");
        key_insert_str(&mut item, self.oshirase.clone(), "oshirase");
        key_insert_json(&mut item, &self.otokoro, "otokoro");
        key_insert_ss(&mut item, self.omotenashi.clone(), "omotenashi");
        key_insert_ss(&mut item, self.oshiharai.clone(), "oshiharai");
        key_insert_json(&mut item, &self.ima, "ima");
        key_insert_str(&mut item, self.hitokoto.clone(), "hitokoto");
        key_insert_datetime(&mut item, &self.kefu_kara, "kefu_kara");
        key_insert_datetime(&mut item, &self.kefu_made, "kefu_made");
        key_insert_ss(&mut item, self.tanamono.clone(), "tanamono");
        key_insert_datetime(&mut item, &self.created_at, "created_at");
        key_insert_datetime(&mut item, &self.updated_at, "updated_at");
        return PutItemInput {
            item: item,
            table_name: table_name(),
            ..Default::default()
        };
    }
}
