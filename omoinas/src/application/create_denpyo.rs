use chrono::{FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::model::denpyo::*;
use crate::model::error::ApplicationError;

#[derive(Deserialize, Debug)]
pub struct Event {
    client_id: String,
    omise_id: String,
    maroudo_id: String,
    id: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub denpyo: Denpyo,
}
pub fn main<DR: DenpyoRepo>(e: Event) -> Result<Response, ApplicationError> {
    let dr = DR::new();
    let denpyo = Denpyo {
        omise_uri: format!("{}/{}", e.client_id, e.omise_id),
        maroudo_id: e.maroudo_id,
        id: e.id,
        shinamono: vec![],
        sum: 0,
        memo: String::from(""),

        created_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
        updated_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
    };
    dr.put(&denpyo)?;

    return Ok(Response { denpyo: denpyo });
}
