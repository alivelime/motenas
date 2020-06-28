#![deny(warnings)]

use log;
use simple_logger;

use lambda_runtime::{error::HandlerError, lambda, Context};

use omoinas::application::reset_omise::{self, Event, Response};
use omoinas::repository::omise::dynamodb::OmiseDb;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(handler)
}

fn handler(e: Event, _: Context) -> Result<Response, HandlerError> {
    return match reset_omise::main::<OmiseDb>(e) {
        Ok(r) => Ok(r),
        Err(_) => Ok(Response { omise: Vec::new() }),
    };
}
