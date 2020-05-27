pub mod model;

use log::{debug, error};

use chrono::{FixedOffset, Utc};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

use crate::model::omise::Omise;

fn init(omise: &mut Omise) {
    debug!("init omise {}", &omise.namae);
    let client = DynamoDbClient::new(Region::ApNortheast1);
    omise.created_at = Utc::now()
        .with_timezone(&FixedOffset::east(9 * 3600))
        .format("%Y%m%d")
        .to_string();
    match client.put_item(omise.put_item()).sync() {
        Ok(_) => {}
        Err(err) => error!("repository::omise::dynamodb::init() {}", err),
    }
}

pub fn get(omise: &mut Omise) -> bool {
    let client = DynamoDbClient::new(Region::ApNortheast1);
    match client.get_item(omise.get_item()).sync() {
        Ok(result) => match result.item {
            Some(item) => {
                omise.from(item);
                return true;
            }
            None => {
                error!("no omise item. {}", omise.namae);
            }
        },
        Err(err) => {
            error!("{}", err);
        }
    }
    init(omise);
    return true;
}
