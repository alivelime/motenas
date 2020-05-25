#![deny(warnings)]

use log::{self, debug, error};
use serde::{Deserialize, Serialize};
use simple_logger;

use lambda_runtime::{error::HandlerError, lambda, Context};

use omoinas::application::hitogata;
use omoinas::omomuki::{self, Omomuki};
use omoinas::repository::cache::dynamodb as cache;
use omoinas::service::cotoha;

#[derive(Deserialize, Debug)]
struct Event {
    text: String,
    chara_id: String,
    id: String,
}

#[derive(Serialize, Debug)]
struct Response {
    r#type: String,
    message: String,
    carousel: Vec<Carousel>,
}
#[derive(Serialize, Debug)]
struct Carousel {
    image: String,
    url: String,
    title: String,
    text: String,
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(handler)
}

fn handler(e: Event, _: Context) -> Result<Response, HandlerError> {
    let chara = hitogata::new(&e.chara_id);
    let token = match cache::get_cotoha_token() {
        Some(t) => t,
        None => {
            return Ok(Response {
                r#type: String::from("message"),
                message: (chara.kaeshi.error.token)(),
                carousel: Vec::new(),
            });
        }
    };

    let tree = match cotoha::parse(&token, &e.text) {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err);
            return Ok(Response {
                r#type: String::from("message"),
                message: (chara.kaeshi.error.parse)(),
                carousel: Vec::new(),
            });
        }
    };
    debug!("{:#?}", tree.chunks);

    /*
    let sentence_type = match cotoha::api::get_sentence_type(&token, &e.text) {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err);
            return Ok(Response {
                message: (chara.kaeshi.error.sentence)(),
            });
        }
    };
    */

    let uke = Omomuki::new(&tree);
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
