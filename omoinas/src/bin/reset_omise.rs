#![deny(warnings)]

use log;
use simple_logger;

use lambda_http::{lambda, Body, IntoResponse, Request, Response};
use lambda_runtime::{error::HandlerError, Context};

use omoinas::application::reset_omise::{self};
use omoinas::repository::omise::dynamodb::OmiseDb;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(handler)
}

fn handler(r: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let event = match r.body() {
        Body::Text(text) => serde_json::from_str(text.as_str()).unwrap(),
        _ => {
            return Ok(Response::builder()
                .status(404)
                .body(String::from("request body is not text."))
                .expect("failed to render response"))
        }
    };
    return match reset_omise::main::<OmiseDb>(event) {
        Ok(res) => Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&res).unwrap())
            .expect("failed to render response")),
        Err(err) => Ok(Response::builder()
            .status(404)
            .body(err)
            .expect("failed to render response")),
    };
}
