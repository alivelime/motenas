pub mod model;

use log::error;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

use crate::model::denpyo::{Denpyo, DenpyoRepo};
use crate::model::error::ApplicationError;

pub struct DenpyoDb {}

impl DenpyoRepo for DenpyoDb {
    fn new() -> DenpyoDb {
        return DenpyoDb {};
    }
    /*
        match client
            .query(QueryInput {
                table_name: model::table_name(),
                key_condition_expression: Some(String::from("omise_uri = :omise_uri AND maroudo_id = :maroudo_id")),
                expression_attribute_values: Some(key),
                filter_expression: Some(String::from("completed_at <> :completed_at")),
                ..Default::default(),

    // use std::default::Default;
    // QueryInput
            })
            .sync()
            */
    fn get(
        &self,
        omise_uri: String,
        maroudo_id: String,
    ) -> Result<Option<Denpyo>, ApplicationError> {
        let mut denpyo = Denpyo::new(omise_uri, maroudo_id);
        let client = DynamoDbClient::new(Region::ApNortheast1);
        return match client.get_item(denpyo.get_item()).sync() {
            Ok(result) => match result.item {
                Some(item) => {
                    denpyo.from(item);
                    Ok(Some(denpyo))
                }
                None => {
                    // なくても良い
                    Ok(None)
                }
            },
            Err(err) => {
                error!(
                    "repository::denpyo::dynamodb::get() client.get_item() : {}",
                    err
                );
                Err(ApplicationError::DynamoDbGetItem(err))
            }
        };
    }
    fn put(&self, denpyo: &Denpyo) -> Result<bool, ApplicationError> {
        let client = DynamoDbClient::new(Region::ApNortheast1);
        return match client.put_item(denpyo.put_item()).sync() {
            Ok(_) => Ok(true),
            Err(err) => {
                error!("repository::denpyo::dynamodb::put() {}", err);
                Err(ApplicationError::DynamoDbPutItem(err))
            }
        };
    }
}
