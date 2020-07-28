use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

pub trait DenpyoRepo {
    fn new() -> Self;
    fn put(&self, denpyo: &Denpyo) -> bool;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shina {
    pub namae: String,
    pub neuchi: i32,
    pub num: i32,
    pub tax_rate: u32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Denpyo {
    pub omise_uri: String,
    pub id: String,
    pub maroudo_id: String,

    pub shinamono: Vec<Shina>,
    pub sum: i64,

    pub created_at: DateTime<FixedOffset>,
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub updated_at: DateTime<FixedOffset>,
}
