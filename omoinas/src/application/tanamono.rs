use lambda_runtime::error::HandlerError;
use serde::{Deserialize, Serialize};

use crate::application::hitogata;
use crate::model::omise::{OmiseRepo, Status};

#[derive(Deserialize, Debug)]
pub struct Event {
    text: String,
    chara_id: String,
    id: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    r#type: String,
    message: String,
}
pub fn main<OR: OmiseRepo>(e: Event) -> Result<Response, HandlerError> {
    let mut chara = hitogata::new(&e.chara_id);
    let or = OR::new();

    let res = match e.text.as_str() {
        "休み" => or.ima(&mut chara.omise, Status::Yasumi),
        "空いてる" => or.ima(&mut chara.omise, Status::Hima),
        "ぼちぼち" => or.ima(&mut chara.omise, Status::Bochibochi),
        "混んでる" => or.ima(&mut chara.omise, Status::Isogashi),
        "貸切" => or.ima(&mut chara.omise, Status::Kashikiri),
        _ => false,
    };

    return Ok(Response {
        r#type: String::from("message"),
        message: match res {
            true => format!("設定しました"),
            false => format!("エラーです"),
        },
    });
}
