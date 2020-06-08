#![deny(warnings)]

use log;
use simple_logger;

use lambda_http::{lambda, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};

use omoinas::application::get_omise::{self, Event};
use omoinas::repository::omise::dynamodb::OmiseDb;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(handler)
}

fn handler(e: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    return match get_omise::main::<OmiseDb>(Event {
        client_id: e.path_parameters().get("clientId").unwrap().to_string(),
        omise_id: e.path_parameters().get("omiseId").unwrap().to_string(),
    }) {
        Ok(r) => Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&r).unwrap())
            .expect("failed to render response")),
        Err(err) => Ok(Response::builder()
            .status(404)
            .body(err)
            .expect("failed to render response")),
    };
}
