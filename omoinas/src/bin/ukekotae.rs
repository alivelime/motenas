#![deny(warnings)]

use log;
use simple_logger;

use lambda_runtime::{error::HandlerError, lambda, Context};

use omoinas::application::ukekotae::{self, Event, Response};
use omoinas::repository::cache::dynamodb::DynamoDb;
use omoinas::repository::omise::dynamodb::OmiseDb;
use omoinas::service::parser::cotoha::Cotoha;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(handler)
}

fn handler(e: Event, _: Context) -> Result<Response, HandlerError> {
    return match ukekotae::main::<Cotoha, DynamoDb, OmiseDb>(e) {
        Ok(r) => Ok(r),
        Err(err) => Ok(Response {
            r#type: String::from("message"),
            message: err,
            carousel: Vec::new(),
        }),
    };
}
