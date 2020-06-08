use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::mono::Mono;

pub trait OmiseRepo {
    fn new() -> Self;
    fn get(&self, client_id: &String, omise_id: &String) -> Result<Omise, String>;
    fn put(&self, omise: &Omise) -> bool;
    fn ima(&self, omise: &mut Omise, status: Status) -> bool;
}

pub struct Client {
    pub id: &'static str,
    pub name: &'static str,
    pub hp: &'static str,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Omise {
    pub client_id: String,
    pub omise_id: String,
    pub namae: String,
    pub url: String,
    pub yotei: String,
    pub otokoro: Address,
    #[serde(skip)]
    pub oshinagaki: Vec<Mono>,

    pub ima: Status,
    pub kefu_kara: DateTime<FixedOffset>,
    pub kefu_made: DateTime<FixedOffset>,

    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
impl Omise {
    pub fn new(client_id: String, omise_id: String) -> Omise {
        return Omise {
            client_id: client_id,
            omise_id: omise_id,
            namae: String::from(""),
            url: String::from(""),
            yotei: String::from(""),
            otokoro: Address::new(),
            oshinagaki: Vec::new(),

            ima: Status::Wakaran,
            kefu_kara: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),
            kefu_made: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),

            created_at: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),
            updated_at: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),
        };
    }
    pub fn menu(&self) -> Vec<&Mono> {
        return self.oshinagaki.iter().collect::<Vec<&Mono>>();
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Status {
    Wakaran,
    Yasumi,
    Hima,
    Bochibochi,
    Isogashi,
    Kashikiri,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Address {
    pub country: String,
    pub postcode: u32,
    pub prefcode: u32,
    pub city: String,
    pub street: String,
    pub building: String,
    pub access: String,
}
impl Address {
    pub fn new() -> Address {
        return Address {
            country: String::from("jpn"),
            postcode: 1100001,
            prefcode: 13,
            city: String::from("千代田区"),
            street: String::from("千代田1-1"),
            building: String::from("デフォルト"),
            access: String::from("デフォルト"),
        };
    }
}
