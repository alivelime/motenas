use chrono::{DateTime, FixedOffset, Utc};

pub struct Maroudo {
    pub line: String,
    pub namae: String,

    pub createdAt: DateTime<FixedOffset>,
    pub updatedAt: DateTime<FixedOffset>,
}

pub struct shina {
    pub namae: String,
    pub neuchi: i32,
    pub num: i32,
    pub tax_rate: u32,
    pub status: u64,
}
pub struct Denpyo {
    pub omiseId: String,
    pub maroudoId: String,
    pub ima: bool,

    pub shinamono: Vec<shina>,
    pub sum: u64,

    pub createdAt: DateTime<FixedOffset>,
    pub completedAt: DateTime<FixedOffset>,
    pub updatedAt: DateTime<FixedOffset>,
}
