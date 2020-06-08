pub mod model;

use log::error;

use chrono::{FixedOffset, Utc};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

use crate::model::omise::{Omise, OmiseRepo, Status};

pub struct OmiseDb {}

impl OmiseRepo for OmiseDb {
    fn new() -> OmiseDb {
        return OmiseDb {};
    }
    fn ima(&self, omise: &mut Omise, ima: Status) -> bool {
        omise.ima = ima;
        omise.updated_at = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
        return self.put(omise);
    }
    fn get(&self, client_id: &String, omise_id: &String) -> Result<Omise, String> {
        let mut omise = Omise::new(client_id.clone(), omise_id.clone());
        let client = DynamoDbClient::new(Region::ApNortheast1);
        match client.get_item(omise.get_item()).sync() {
            Ok(result) => match result.item {
                Some(item) => {
                    omise.from(item);
                    return Ok(omise);
                }
                None => {
                    let err = format!("no omise item. {}", omise.namae);
                    error!(
                        "repository::omise::dynamodb::get() client.get_item() : {}",
                        err
                    );
                    return Err(err);
                }
            },
            Err(err) => {
                error!(
                    "repository::omise::dynamodb::get() client.get_item() : {}",
                    err
                );
                return Err(err.to_string());
            }
        }
    }
    fn put(&self, omise: &Omise) -> bool {
        let client = DynamoDbClient::new(Region::ApNortheast1);
        match client.put_item(omise.put_item()).sync() {
            Ok(_) => {}
            Err(err) => {
                error!("repository::omise::dynamodb::put() {}", err);
                return false;
            }
        }
        return true;
    }
}
