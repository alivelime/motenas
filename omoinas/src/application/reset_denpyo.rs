use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::model::denpyo::*;

#[derive(Deserialize, Debug)]
pub struct Event {
    client_id: Option<String>,
    omise_id: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub denpyo: Vec<Denpyo>,
}
pub fn main<DR: DenpyoRepo>(_: Event) -> Result<Response, String> {
    let dr = DR::new();
    let denpyo = vec![Denpyo {
        omise_uri: String::from("tokishirazu.llc/passengers"),
        maroudo_id: String::from("Ube2da389e4c223405286c03a32fefcb6"),
        id: String::from("18"),

        shinamono: vec![
            Shina {
                namae: String::from("バニラアイス"),
                neuchi: 270,
                num: 1,
                tax_rate: 10,
                status: Status::Open,
                memo: String::from(""),
            },
            Shina {
                namae: String::from("ホットコーヒー"),
                neuchi: 230,
                num: 1,
                tax_rate: 10,
                status: Status::Open,
                memo: String::from("砂糖とミルクはいらないです"),
            },
        ],
        sum: 550,

        created_at: DateTime::parse_from_rfc3339("2020-07-28T00:00:00+09:00").unwrap(),
        updated_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
    }];

    for d in &denpyo {
        dr.put(d);
    }

    return Ok(Response { denpyo: denpyo });
}
