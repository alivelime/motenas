pub mod model;

use log::error;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

use crate::model::error::ApplicationError;
use crate::model::omise::{Omise, OmiseRepo};

pub struct OmiseDb {}

impl OmiseRepo for OmiseDb {
    fn new() -> OmiseDb {
        return OmiseDb {};
    }
    fn get(&self, client_id: &String, omise_id: &String) -> Result<Omise, ApplicationError> {
        let mut omise = Omise::new(client_id.clone(), omise_id.clone());
        let client = DynamoDbClient::new(Region::ApNortheast1);
        return match client.get_item(omise.get_item()).sync() {
            Ok(result) => match result.item {
                Some(item) => {
                    omise.from(item);
                    Ok(omise)
                }
                None => {
                    let err = format!("no omise item. {}", &omise.namae);
                    error!(
                        "repository::omise::dynamodb::get() client.get_item() : {}",
                        err
                    );
                    Err(ApplicationError::NotFound(omise.namae))
                }
            },
            Err(err) => {
                error!(
                    "repository::omise::dynamodb::get() client.get_item() : {}",
                    err
                );
                Err(ApplicationError::DynamoDbGetItem(err))
            }
        };
    }
    fn put(&self, omise: &Omise) -> Result<bool, ApplicationError> {
        let client = DynamoDbClient::new(Region::ApNortheast1);
        return match client.put_item(omise.put_item()).sync() {
            Ok(_) => Ok(true),
            Err(err) => {
                error!("repository::omise::dynamodb::put() {}", err);
                Err(ApplicationError::DynamoDbPutItem(err))
            }
        };
    }
}
