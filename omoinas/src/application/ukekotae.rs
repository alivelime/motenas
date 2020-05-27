use lambda_runtime::error::HandlerError;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::application::hitogata;
use crate::model::cache::Cache;
use crate::model::parser::Parser;
use crate::omomuki::{self, Omomuki};

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
    carousel: Vec<Carousel>,
}
#[derive(Serialize, Debug)]
pub struct Carousel {
    image: String,
    url: String,
    title: String,
    text: String,
}

pub fn main<P: Parser, C: Cache>(e: Event) -> Result<Response, HandlerError> {
    let chara = hitogata::new(&e.chara_id);
    let (ok, tree) = P::parse::<C>(&e.text);
    if !ok {
        return Ok(Response {
            r#type: String::from("message"),
            message: (chara.kaeshi.error.parse)(),
            carousel: Vec::new(),
        });
    };

    let omomuki = Omomuki::new(&tree);
    debug!("{:#?}", omomuki);

    let uke = omomuki.tumori();
    debug!("{:#?}", &uke);
    let kotae = uke.kotafu(&chara);
    debug!("{:#?}", &kotae);

    return Ok(match kotae.get_kotae(&chara) {
        omomuki::Result::Message(text) => Response {
            r#type: String::from("message"),
            message: text,
            carousel: Vec::new(),
        },
        omomuki::Result::Mono(message, monos) => Response {
            r#type: String::from("carousel"),
            message: message,
            carousel: monos
                .iter()
                .map(|m| Carousel {
                    title: String::from(m.namae),
                    text: format!("{}円\n{}", m.neuchi, m.comment),
                    image: String::from(m.image),
                    url: String::from(m.url),
                })
                .collect(),
        },
    });
}
