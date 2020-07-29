use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

pub trait DenpyoRepo {
    fn new() -> Self;
    fn put(&self, denpyo: &Denpyo) -> bool;
    fn get(&self, omsier_uri: String, maroudo_id: String) -> Result<Option<Denpyo>, String>;
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Status {
    Open,
    Processing,
    Pending,
    Close,
    Complete,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shina {
    pub namae: String,
    pub neuchi: i32,
    pub num: i32,
    pub tax_rate: i32,
    pub status: Status,
    pub memo: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Denpyo {
    #[serde(rename = "omiseUri")]
    pub omise_uri: String,
    #[serde(rename = "maroudoId")]
    pub maroudo_id: String,
    pub id: String,

    pub shinamono: Vec<Shina>,
    pub sum: i64,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,
}

impl Denpyo {
    pub fn new(omise_uri: String, maroudo_id: String) -> Denpyo {
        return Denpyo {
            omise_uri: omise_uri,
            maroudo_id: maroudo_id,
            id: String::from(""),
            shinamono: vec![],
            sum: 0,
            created_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
            updated_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
        };
    }
}
