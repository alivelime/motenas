#![deny(warnings)]

use log;
use simple_logger;

use lambda_http::{lambda, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};

use omoinas::application::get_denpyo;
use omoinas::application::get_omise;
use omoinas::repository::denpyo::dynamodb::DenpyoDb;
use omoinas::repository::omise::dynamodb::OmiseDb;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(r: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    let res: Result<String, String> = match c.function_name.split("-").collect::<Vec<&str>>()[2] {
        "getOmise" => match get_omise::main::<OmiseDb>(get_omise::Event {
            client_id: r.path_parameters().get("clientId").unwrap().to_string(),
            omise_id: r.path_parameters().get("omiseId").unwrap().to_string(),
        }) {
            Ok(res) => Ok(serde_json::to_string(&res).unwrap()),
            Err(err) => Err(err),
        },
        "getDenpyo" => {
            match get_denpyo::main::<DenpyoDb>(get_denpyo::Event {
                client_id: r.path_parameters().get("clientId").unwrap().to_string(),
                omise_id: r.path_parameters().get("omiseId").unwrap().to_string(),
                maroudo_id: r.path_parameters().get("maroudoId").unwrap().to_string(),
            }) {
                Ok(res) => Ok(serde_json::to_string(&res).unwrap()),
                Err(err) => Err(err),
            }
        }
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
                match r.headers().get("Origin") {
                    Some(o) => o.to_str().unwrap(),
                    None => "",
                },
            )
            .header(
                "Access-Control-Allow-Headers",
                "Origin,Authorization,Accept,X-Requested-With",
            )
            .body(res)
            .expect("failed to render response")),
        Err(err) => Ok(Response::builder()
            .status(404)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
            .header("Access-Control-Allow-Credential", "true")
            .header(
                "Access-Control-Allow-Origin",
                match r.headers().get("Origin") {
                    Some(o) => o.to_str().unwrap(),
                    None => "",
                },
            )
            .header(
                "Access-Control-Allow-Headers",
                "Origin,Authorization,Accept,X-Requested-With",
            )
            .body(err)
            .expect("failed to render response")),
    };
}
