use chrono::{DateTime, FixedOffset};

use crate::model::mono::Mono;

pub struct Client {
    pub id: &'static str,
    pub namae: &'static str,
    pub hp: &'static str,
}
pub struct Omise {
    pub id: String,
    pub client: Client,
    pub namae: &'static str,
    pub hp: &'static str,
    pub yotei: &'static str,
    pub menu: Vec<Mono>,
    pub status: Status,

    pub created_at: DateTime<FixedOffset>,
}

impl Omise {
    pub fn menu(&self) -> Vec<&Mono> {
        return self.menu.iter().collect::<Vec<&Mono>>();
    }
}

pub enum Status {
    Yasumi,
    Hima,
    Bochibochi,
    Isogashi,
    Kashikiri,
}

/*
pub struct Address {
    namae: String,
    country: String,
    postcode: u32,
    prefcode: u32,
    city: String,
    address1: String,
    address2: String,
    access: String,
    lat: f64,
    lng: f64,
}
*/
