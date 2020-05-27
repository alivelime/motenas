#![deny(warnings)]

use log;
use simple_logger;

use lambda_runtime::{error::HandlerError, lambda, Context};

use omoinas::application::ukekotae::{self, Event, Response};
use omoinas::repository::cache::dynamodb::DynamoDb;
use omoinas::service::parser::cotoha::Cotoha;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(handler)
}

fn handler(e: Event, _: Context) -> Result<Response, HandlerError> {
    return ukekotae::main::<Cotoha, DynamoDb>(e);
}
