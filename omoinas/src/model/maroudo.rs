use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};

pub struct Maroudo {
    pub line: String,
    pub namae: String,
    pub chara: HashMap<String, String>,

    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
