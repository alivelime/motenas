#![deny(warnings)]

use log;
use simple_logger;

use lambda_runtime::{error::HandlerError, lambda, Context};

use omoinas::application::tanamono::{self, Event, Response};
use omoinas::repository::omise::dynamodb::OmiseDb;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(e: Event, _: Context) -> Result<Response, HandlerError> {
    return match tanamono::main::<OmiseDb>(e) {
        Ok(r) => Ok(r),
        Err(err) => Ok(Response {
            ok: false,
            message: err,
        }),
    };
}
