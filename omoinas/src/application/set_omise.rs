use log::error;
use std::collections::HashSet;

use chrono::{DateTime, Datelike, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::model::error::ApplicationError;
use crate::model::omise::{Ima, Links, OmiseRepo};

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "omiseId")]
    omise_id: String,
    #[serde(rename = "tanamonoId")]
    tanamono_id: String,

    namae: String,
    ima: Vec<Ima>,
    hitokoto: String,
    #[serde(rename = "kefuKara")]
    kefu_kara: i32,
    #[serde(rename = "kefuMade")]
    kefu_made: i32,
    omotenashi: HashSet<String>,
    oshiharai: HashSet<String>,
    yotei: String,
    oshirase: String,

    hp: String,
    twitter: String,
    facebook: String,
    instagram: String,
    line: String,

    postcode: u32,
    prefcode: u32,
    city: String,
    street: String,
    building: String,
}

#[derive(Serialize, Debug)]
pub struct Response {}

pub fn main<OR: OmiseRepo>(e: Event) -> Result<Response, ApplicationError> {
    let or = OR::new();
    let mut omise = or.get(&e.client_id, &e.omise_id)?;

    // 権限チェック
    if !omise.tanamono.contains(&e.tanamono_id) {
        error!(
            "{}/{} is not in {} tanamono.",
            &e.client_id, &e.omise_id, &e.tanamono_id
        );
        return Err(ApplicationError::PermissionDenied);
    }

    omise.namae = e.namae;
    omise.ima = e.ima;
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
    omise.oshirase = e.oshirase;
    omise.link = Links {
        hp: e.hp,
        twitter: e.twitter,
        facebook: e.facebook,
        instagram: e.instagram,
        line: e.line,
    };
    omise.otokoro.postcode = e.postcode;
    omise.otokoro.prefcode = e.prefcode;
    omise.otokoro.city = e.city;
    omise.otokoro.street = e.street;
    omise.otokoro.building = e.building;
    omise.updated_at = now;

    or.put(&omise)?;

    return Ok(Response {});
}
