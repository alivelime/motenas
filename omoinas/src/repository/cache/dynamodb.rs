pub mod model;

use log::{debug, error};

use chrono::{FixedOffset, Utc};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

use crate::model::setting::CotohaToken;
use crate::service::cotoha;

pub fn get_cotoha_token() -> Option<String> {
    let today = Utc::now()
        .with_timezone(&FixedOffset::east(9 * 3600))
        .format("%Y%m%d")
        .to_string();
    let client = DynamoDbClient::new(Region::ApNortheast1);
    match client.get_item(CotohaToken::get_item()).sync() {
        Ok(result) => match result.item {
            Some(item) => {
                if let Some(ct) = CotohaToken::from(item) {
                    if ct.date == today {
                        return Some(ct.token());
                    }
                    debug!("expired cotoha token.");
                }
            }
            None => {
                debug!("no cotoha token.()");
            }
        },
        Err(err) => {
            error!("{}", err);
        }
    };

    debug!("get_cotoha_token()");
    match cotoha::api::get_access_token() {
        Ok(t) => {
            match client.put_item(CotohaToken::new(&t).put_item()).sync() {
                Ok(_) => {}
                Err(err) => error!("{}", err),
            }
            return Some(t);
        }
        Err(err) => {
            error!("{}", err);
            return None;
        }
    }
}
