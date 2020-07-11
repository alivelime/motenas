use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::omise::{Address as MAddress, Ima, Links, Omise as MOmise, OmiseRepo};

#[derive(Deserialize, Debug)]
pub struct Event {
    pub client_id: String,
    pub omise_id: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    omise: Omise,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Omise {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "omiseId")]
    pub omise_id: String,
    pub namae: String,
    pub link: Links,
    pub yotei: String,
    pub oshirase: String,
    pub otokoro: Address,

    pub omotenashi: HashSet<String>,
    pub oshiharai: HashSet<String>,

    pub ima: Vec<Ima>,
    pub hitokoto: String,
    #[serde(rename = "kefuKara")]
    pub kefu_kara: DateTime<FixedOffset>,
    #[serde(rename = "kefuMade")]
    pub kefu_made: DateTime<FixedOffset>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Address {
    pub country: String,
    pub postcode: u32,
    #[serde(rename = "forMap")]
    pub for_map: String,
    pub todofuken: String,
    pub prefcode: u32,
    pub city: String,
    pub street: String,
    pub building: String,
    pub access: String,
}

pub fn main<OR: OmiseRepo>(e: Event) -> Result<Response, String> {
    let or = OR::new();

    return match or.get(&e.client_id, &e.omise_id) {
        Ok(omise) => Ok(Response {
            omise: Omise::from(omise),
        }),
        Err(err) => Err(err),
    };
}

impl Omise {
    pub fn from(m: MOmise) -> Omise {
        return Omise {
            client_id: m.client_id,
            omise_id: m.omise_id,
            namae: m.namae,
            link: m.link,
            yotei: m.yotei,
            oshirase: m.oshirase,
            otokoro: Address::from(m.otokoro),
            omotenashi: m.omotenashi,
            oshiharai: m.oshiharai,

            ima: m.ima,
            hitokoto: m.hitokoto,
            kefu_kara: m.kefu_kara,
            kefu_made: m.kefu_made,

            created_at: m.created_at,
            updated_at: m.updated_at,
        };
    }
}
impl Address {
    pub fn from(m: MAddress) -> Address {
        return Address {
            for_map: m.for_map(),
            todofuken: m.todofuken(),

            country: m.country,
            postcode: m.postcode,
            prefcode: m.prefcode,
            city: m.city,
            street: m.street,
            building: m.building,
            access: m.access,
        };
    }
}
