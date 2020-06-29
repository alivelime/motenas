pub mod model;

use log::{error, info};

use chrono::{FixedOffset, Utc};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb as _, DynamoDbClient};

use crate::model::cache::Cache;
use crate::model::parser::Parser;
use crate::model::setting::ParserToken;

pub struct DynamoDb {}

impl Cache for DynamoDb {
    fn get_parser_token<P: Parser>() -> Option<String> {
        let today = Utc::now()
            .with_timezone(&FixedOffset::east(9 * 3600))
            .format("%Y%m%d")
            .to_string();
        let client = DynamoDbClient::new(Region::ApNortheast1);
        match client.get_item(ParserToken::get_item()).sync() {
            Ok(result) => match result.item {
                Some(item) => {
                    if let Some(ct) = ParserToken::from(item) {
                        if ct.date == today {
                            return Some(ct.token());
                        }
                        info!("expired cotoha token.");
                    }
                }
                None => {
                    info!("no cotoha token.()");
                }
            },
            Err(err) => {
                error!("{}", err);
            }
        };

        info!("get_cotoha_token()");
        match P::get_access_token() {
            Ok(t) => {
                match client.put_item(ParserToken::new(&t).put_item()).sync() {
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
}
