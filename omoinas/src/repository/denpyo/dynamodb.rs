pub mod model;

use log::error;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

use crate::model::denpyo::{Denpyo, DenpyoRepo};

pub struct DenpyoDb {}

impl DenpyoRepo for DenpyoDb {
    fn new() -> DenpyoDb {
        return DenpyoDb {};
    }
    fn put(&self, denpyo: &Denpyo) -> bool {
        let client = DynamoDbClient::new(Region::ApNortheast1);
        match client.put_item(denpyo.put_item()).sync() {
            Ok(_) => {}
            Err(err) => {
                error!("repository::denpyo::dynamodb::put() {}", err);
                return false;
            }
        }
        return true;
    }
}
