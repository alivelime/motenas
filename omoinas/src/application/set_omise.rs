use log::error;
use std::collections::HashSet;

use chrono::{DateTime, Datelike, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::model::omise::{OmiseRepo, Status};

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "omiseId")]
    omise_id: String,
    #[serde(rename = "userId")]
    user_id: String,

    namae: String,
    ima: i32,
    hitokoto: String,
    #[serde(rename = "kefuKara")]
    kefu_kara: i32,
    #[serde(rename = "kefuMade")]
    kefu_made: i32,
    omotenashi: HashSet<String>,
    yotei: String,
    url: String,
    postcode: u32,
    prefcode: u32,
    city: String,
    street: String,
    building: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub message: String,
}

pub fn main<OR: OmiseRepo>(e: Event) -> Result<Response, String> {
    let or = OR::new();
    let mut omise = match or.get(&e.client_id, &e.omise_id) {
        Ok(o) => o,
        Err(e) => {
            error!("{}", e);
            return Err(e.to_string());
        }
    };

    // 権限チェック
    if !omise.tanamono.contains(&e.user_id) {
        error!(
            "{}/{} is not in {} tanamono.",
            &e.client_id, &e.omise_id, &e.user_id
        );
        return Err(format!("permission denied."));
    }

    omise.namae = e.namae;
    omise.ima = match e.ima {
        0 => Status::Wakaran,
        1 => Status::Yasumi,
        2 => Status::Hima,
        3 => Status::Bochibochi,
        4 => Status::Isogashi,
        5 => Status::Ippai,
        6 => Status::Kashikiri,
        _ => return Err(format!("invalid status. {}", &e.ima)),
    };
    omise.hitokoto = e.hitokoto;

    let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
    omise.kefu_kara = DateTime::parse_from_rfc3339(
        format!(
            "{}T{:>02}:00:00+09:00",
            now.format("%Y-%m-%d").to_string(),
            &e.kefu_kara,
        )
        .as_str(),
    )
    .unwrap();
    omise.kefu_made = DateTime::parse_from_rfc3339(
        format!(
            "{}-{:>02}-{:>02}T{:>02}:00:00+09:00",
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

    omise.omotenashi = e.omotenashi;
    omise.yotei = e.yotei;
    omise.url = e.url;
    omise.otokoro.postcode = e.postcode;
    omise.otokoro.prefcode = e.prefcode;
    omise.otokoro.city = e.city;
    omise.otokoro.street = e.street;
    omise.otokoro.building = e.building;
    omise.updated_at = now;

    if !or.put(&omise) {
        return Err(String::from("dynamo put error."));
    }
    return Ok(Response {
        ok: true,
        message: String::new(),
    });
}
