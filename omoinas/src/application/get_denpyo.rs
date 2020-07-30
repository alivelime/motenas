use serde::{Deserialize, Serialize};

use crate::model::denpyo::*;
use crate::model::error::ApplicationError;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub client_id: String,
    pub omise_id: String,
    pub maroudo_id: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    denpyo: Option<Denpyo>,
}

pub fn main<DR: DenpyoRepo>(e: Event) -> Result<Response, ApplicationError> {
    let dr = DR::new();

    return match dr.get(format!("{}/{}", e.client_id, e.omise_id), e.maroudo_id) {
        Ok(denpyo) => Ok(Response { denpyo: denpyo }),
        Err(err) => Err(err),
    };
}
