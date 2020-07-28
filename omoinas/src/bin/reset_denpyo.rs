#![deny(warnings)]

use log;
use simple_logger;

use lambda_runtime::{error::HandlerError, lambda, Context};

use omoinas::application::reset_denpyo::{self, Event, Response};
use omoinas::repository::denpyo::dynamodb::DenpyoDb;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(e: Event, _: Context) -> Result<Response, HandlerError> {
    return match reset_denpyo::main::<DenpyoDb>(e) {
        Ok(r) => Ok(r),
        Err(_) => Ok(Response { denpyo: Vec::new() }),
    };
}
