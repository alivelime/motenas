use log::{debug, error};

use chrono::{FixedOffset, Utc};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

use crate::model::omise::{Omise, Status};

fn init(&mut omise: Omise) {
    match client.put_item(omise.put_item()).sync() {
        Ok(_) => {}
        Err(err) => error!("{}", err),
    }
}

pub fn get_status(&mut omise: Omise) -> Option<Status> {
    let client = DynamoDbClient::new(Region::ApNortheast1);
    match client.get_item(Omise::get_item()).sync() {
        Ok(result) => match result.item {
            Some(item) => {
                if let Some(ct) = CotohaToken::from(item) {
                    if ct.date() == today {
                        return Some(ct.token());
                    }
                    debug!("expired cotoha token.");
                }
            }
            None => {
                error!("no omise item. {}", namae);
            }
        },
        Err(err) => {
            error!("{}", err);
        }
    }
    init(omise);
}
