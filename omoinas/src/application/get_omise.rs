use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::omise::{Address as MAddress, Omise as MOmise, OmiseRepo};

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
#[derive(Deserialize, Serialize, Debug)]
pub struct Omise {
    pub client_id: String,
    pub omise_id: String,
    pub namae: String,
    pub url: String,
    pub yotei: String,
    pub otokoro: Address,

    pub ima: u32,
    pub hitokoto: String,
    pub kefu_kara: DateTime<FixedOffset>,
    pub kefu_made: DateTime<FixedOffset>,

    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Address {
    pub country: String,
    pub postcode: u32,
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
            r#type: String::from("message"),
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
            url: m.url,
            yotei: m.yotei,
            otokoro: Address::from(m.otokoro),

            ima: m.ima as u32,
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
