use serde::{Deserialize, Serialize};

use crate::application::omise;
use crate::model::omise::OmiseRepo;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub client_id: String,
    pub omise_id: String,
    pub tanamono: String,
    pub command: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub message: String,
}

pub fn main<OR: OmiseRepo>(e: Event) -> Result<Response, String> {
    let mut omise = omise::new::<OR>(format!("{}/{}", &e.client_id, &e.omise_id).as_str());
    let or = OR::new();

    match e.command.as_str() {
        "add" => {
            omise.tanamono.insert(e.tanamono.clone());
        }
        "remove" => {
            omise.tanamono.remove(&e.tanamono);
        }
        _ => {}
    };

    if !or.put(&omise) {
        return Err(String::from("dynamo put error."));
    }
    return Ok(Response {
        ok: true,
        message: String::new(),
    });
}
