#![deny(warnings)]

use log;
use simple_logger;

use lambda_http::{lambda, Body, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};

use omoinas::application::get_omise;
use omoinas::application::reset_omise;
use omoinas::application::set_omise_ima;
use omoinas::repository::omise::dynamodb::OmiseDb;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(handler)
}

fn handler(r: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    let res: Result<String, String> = match c.function_name.as_str() {
        "getOmise" => match get_omise::main::<OmiseDb>(get_omise::Event {
            client_id: r.path_parameters().get("clientId").unwrap().to_string(),
            omise_id: r.path_parameters().get("omiseId").unwrap().to_string(),
        }) {
            Ok(res) => Ok(serde_json::to_string(&res).unwrap()),
            Err(err) => Err(err),
        },
        "setOmiseIma" => match &r.body() {
            Body::Text(text) => {
                let mut event: set_omise_ima::Event = serde_json::from_str(text.as_str()).unwrap();
                event.client_id = r.path_parameters().get("clientId").unwrap().to_string();
                event.omise_id = r.path_parameters().get("omiseId").unwrap().to_string();
                match set_omise_ima::main::<OmiseDb>(event) {
                    Ok(res) => Ok(serde_json::to_string(&res).unwrap()),
                    Err(err) => Err(err),
                }
            }
            _ => Err(String::from("request body is not text.")),
        },
        "resetOmise" => match &r.body() {
            Body::Text(text) => {
                let event = serde_json::from_str(text.as_str()).unwrap();
                match reset_omise::main::<OmiseDb>(event) {
                    Ok(res) => Ok(serde_json::to_string(&res).unwrap()),
                    Err(err) => Err(err),
                }
            }
            _ => Err(String::from("request body is not text.")),
        },
        _ => Err(format!("no such method {}", &c.function_name)),
    };

    return match res {
        Ok(res) => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
            .header("Access-Control-Allow-Credential", "true")
            .header(
                "Access-Control-Allow-Origin",
                r.headers().get("Origin").unwrap(),
            )
            .header(
                "Access-Control-Allow-Headers",
                "Origin,Authorization,Accept,X-Requested-With",
            )
            .body(res)
            .expect("failed to render response")),
        Err(err) => Ok(Response::builder()
            .status(404)
            .body(err)
            .expect("failed to render response")),
    };
}
