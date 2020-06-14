use chrono::{DateTime, Datelike, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::application::omise;
use crate::model::omise::{OmiseRepo, Status};

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(skip)]
    pub client_id: String,
    #[serde(skip)]
    pub omise_id: String,
    pub ima: u32,
    pub hitokoto: String,

    pub kefu_kara: u32,
    pub kefu_made: u32,
}

#[derive(Serialize, Debug)]
pub struct Response {}
pub fn main<OR: OmiseRepo>(e: Event) -> Result<Response, String> {
    let mut omise = omise::new::<OR>(format!("{}/{}", &e.client_id, &e.omise_id).as_str());
    let or = OR::new();

    omise.ima = match e.ima {
        0 => Status::Wakaran,
        1 => Status::Yasumi,
        2 => Status::Hima,
        3 => Status::Bochibochi,
        4 => Status::Isogashi,
        5 => Status::Kashikiri,
        _ => return Err(format!("invalid status. {}", e.ima)),
    };
    omise.hitokoto = e.hitokoto;

    let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
    omise.kefu_kara = DateTime::parse_from_rfc3339(
        format!(
            "{}T{}:00:00+09:00",
            now.format("%Y-%m-%d").to_string(),
            e.kefu_kara,
        )
        .as_str(),
    )
    .unwrap();
    omise.kefu_made = DateTime::parse_from_rfc3339(
        format!(
            "{}-{}-{}T{}:00:00+09:00",
            now.year(),
            now.month(),
            now.day() + if e.kefu_made < 24 { 0 } else { 1 },
            if e.kefu_made < 24 {
                e.kefu_made
            } else {
                e.kefu_made - 24
            }
        )
        .as_str(),
    )
    .unwrap();
    omise.updated_at = now;

    if !or.put(&omise) {
        return Err(String::from("dynamo put error."));
    }
    return Ok(Response {});
}
