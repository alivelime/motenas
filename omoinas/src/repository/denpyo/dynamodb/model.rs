use std::collections::HashMap;
use std::default::Default;
use std::env;

use rusoto_dynamodb::{AttributeValue, GetItemInput, PutItemInput};

use crate::model::denpyo::Denpyo;
use crate::util::dynamodb::*;

pub fn table_name() -> String {
    return format!("{}_{}", env::var("ENV").unwrap(), "denpyo");
}

impl Denpyo {
    pub fn from(&mut self, item: HashMap<String, AttributeValue>) -> bool {
        item_from_str(&item, &mut self.omise_uri, "omise_uri");
        item_from_str(&item, &mut self.maroudo_id, "maroudo_id");
        item_from_str(&item, &mut self.id, "id");
        item_from_json(&item, &mut self.shinamono, "shinamono");
        item_from_num(&item, &mut self.sum, "sum");
        item_from_datetime(&item, &mut self.created_at, "created_at");
        item_from_datetime(&item, &mut self.updated_at, "updated_at");
        return true;
    }

    pub fn get_item(&self) -> GetItemInput {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key_insert_str(&mut key, self.omise_uri.clone(), "omise_uri");
        key_insert_str(&mut key, self.maroudo_id.clone(), "maroudo_id");
        return GetItemInput {
            key: key,
            table_name: table_name(),
            ..Default::default()
        };
    }
    pub fn put_item(&self) -> PutItemInput {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        key_insert_str(&mut item, self.omise_uri.clone(), "omise_uri");
        key_insert_str(&mut item, self.maroudo_id.clone(), "maroudo_id");
        key_insert_str(&mut item, self.id.clone(), "id");
        key_insert_json(&mut item, &self.shinamono, "shinamono");
        key_insert_num(&mut item, self.sum, "sum");
        key_insert_datetime(&mut item, &self.created_at, "created_at");
        key_insert_datetime(&mut item, &self.updated_at, "updated_at");

        return PutItemInput {
            item: item,
            table_name: table_name(),
            ..Default::default()
        };
    }
}
