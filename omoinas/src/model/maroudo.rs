use chrono::{DateTime, FixedOffset, Utc};

pub struct Maroudo {
    pub line: String,
    pub namae: String,

    pub createdAt: DateTime<FixedOffset>,
    pub updatedAt: DateTime<FixedOffset>,
}

pub struct Denpyo {
    pub omiseId: String,
    pub id: u64,
    pub maroudoId: String,

    pub mono: Vec<(String, u64, u64, String)>,
    pub sum: u64,
    pub status: u64,

    pub createdAt: DateTime<FixedOffset>,
    pub updatedAt: DateTime<FixedOffset>,
}
