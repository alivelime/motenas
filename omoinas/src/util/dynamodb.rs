use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use rusoto_dynamodb::AttributeValue;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn item_from_str(item: &HashMap<String, AttributeValue>, m: &mut String, n: &str) {
    if item.get(n).is_some() && item[n].s.is_some() {
        *m = item[n].s.as_ref().unwrap().to_string();
    }
}
pub fn item_from_json<T>(item: &HashMap<String, AttributeValue>, m: &mut T, n: &str)
where
    T: DeserializeOwned,
{
    if item.get(n).is_some() && item[n].s.is_some() {
        *m = serde_json::from_str(item[n].s.as_ref().unwrap().as_str()).unwrap();
    }
}
pub fn item_from_datetime(
    item: &HashMap<String, AttributeValue>,
    m: &mut DateTime<FixedOffset>,
    n: &str,
) {
    if item.get(n).is_some() && item[n].s.is_some() {
        *m = DateTime::parse_from_rfc3339(item[n].s.as_ref().unwrap().as_str()).unwrap();
    }
}

pub fn key_insert_str(key: &mut HashMap<String, AttributeValue>, m: String, n: &str) {
    key.insert(
        String::from(n),
        AttributeValue {
            s: Some(m),
            ..Default::default()
        },
    );
}
pub fn key_insert_json<T>(key: &mut HashMap<String, AttributeValue>, m: &T, n: &str)
where
    T: Serialize,
{
    key.insert(
        String::from(n),
        AttributeValue {
            s: Some(serde_json::to_string(m).unwrap()),
            ..Default::default()
        },
    );
}
pub fn key_insert_num(key: &mut HashMap<String, AttributeValue>, m: i32, n: &str) {
    key.insert(
        String::from(n),
        AttributeValue {
            n: Some(m.to_string()),
            ..Default::default()
        },
    );
}

pub fn key_insert_datetime(
    key: &mut HashMap<String, AttributeValue>,
    m: &DateTime<FixedOffset>,
    n: &str,
) {
    key.insert(
        String::from(n),
        AttributeValue {
            s: Some(m.to_rfc3339()),
            ..Default::default()
        },
    );
}
