use std::collections::HashMap;
use std::default::Default;
use std::env;

use rusoto_dynamodb::{AttributeValue, /* GetItemInput , */ PutItemInput};

use crate::model::denpyo::Denpyo;
use crate::util::dynamodb::*;

fn table_name() -> String {
    return format!("{}_{}", env::var("ENV").unwrap(), "denpyo");
}

impl Denpyo {
    pub fn put_item(&self) -> PutItemInput {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        key_insert_str(&mut item, self.omise_uri.clone(), "omise_uri");
        key_insert_str(&mut item, self.id.clone(), "id");
        key_insert_str(&mut item, self.maroudo_id.clone(), "maroudo_id");
        key_insert_json(&mut item, &self.shinamono, "shinamono");
        key_insert_num(&mut item, self.sum, "sum");
        key_insert_datetime(&mut item, &self.created_at, "created_at");
        key_insert_datetime_option(&mut item, &self.completed_at, "completed_at");
        key_insert_datetime(&mut item, &self.updated_at, "updated_at");

        return PutItemInput {
            item: item,
            table_name: table_name(),
            ..Default::default()
        };
    }
}
