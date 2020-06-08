use log::debug;
use serde::{Deserialize, Serialize};

use crate::model::omise::{Omise, OmiseRepo};

#[derive(Deserialize, Debug)]
pub struct Event {
    pub client_id: String,
    pub omise_id: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    r#type: String,
    omise: Omise,
}
pub fn main<OR: OmiseRepo>(e: Event) -> Result<Response, String> {
    debug!("{:#?}", &e);

    let or = OR::new();

    return match or.get(&e.client_id, &e.omise_id) {
        Ok(omise) => Ok(Response {
            r#type: String::from("message"),
            omise: omise,
        }),
        Err(err) => Err(err),
    };
}
