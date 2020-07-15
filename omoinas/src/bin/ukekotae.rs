#![deny(warnings)]

use log;
use simple_logger;

use lambda_runtime::{error::HandlerError, lambda, Context};

use omoinas::application::ukekotae::{self, Event, Response};
use omoinas::repository::cache::dynamodb::DynamoDb;
use omoinas::repository::omise::dynamodb::OmiseDb;
use omoinas::service::parser::cotoha::Cotoha;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(e: Event, _: Context) -> Result<Response, HandlerError> {
    return match ukekotae::main::<Cotoha, DynamoDb, OmiseDb>(e) {
        Ok(r) => Ok(r),
        Err(err) => Ok(Response {
            r#type: String::from("message"),
            message: err,
            name: String::from("エラー"),
            icon: String::from("https://obs.line-scdn.net/0hkqtt2knTNF5JDRyRjohLCWlQPzx6bypVa2t8PGkNbm1sNXNmc2h_bWxfbTkwPHIKID95aAINYjxsPncNMGoobGlaajxn/f256x256"),
            carousel: Vec::new(),
        }),
    };
}
