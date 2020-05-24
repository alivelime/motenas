use log::debug;

use crate::model::mono::Mono;
use crate::model::omomuki::*;
use crate::service::cotoha;
use crate::Tumori;

pub mod aisatsu;
pub mod kitanai;
pub mod onegai;
pub mod shirase;
pub mod tawainai;
pub mod toikake;
pub mod wakaran;

#[derive(Clone, Debug)]
pub enum Omomuki {
    Suru(Suru),
    Shitai(Suru),
    Shite(Suru),
    Taigen(Taigen),
    Keiyou(Keiyou),
    Dearu(Dearu),
    Ocha(Ocha),
    Tawainai(Tawainai),
}
pub enum Result {
    Message(String),
    Mono(String, Vec<Mono>),
}

impl Omomuki {
    pub fn new(tree: &cotoha::ParseObjects) -> Box<dyn Tumori> {
        // NGワードは弾く
        if let Some(a) = kitanai::new(&tree) {
            return a;
        }

        // まんず、あいさつかどうか調べるべ
        if let Some(a) = aisatsu::new(&tree) {
            return a;
        }

        let omomuki = get_omomuki(&tree);
        debug!("{:#?}", omomuki);

        // お願い
        if let Some(t) = onegai::new(&omomuki) {
            return t;
        }

        // 問いかけ
        if let Some(t) = toikake::new(&omomuki) {
            return t;
        }

        // 知らせる
        if let Some(t) = shirase::new(&omomuki) {
            return t;
        }

        // たわいない
        if let Some(t) = tawainai::new(&omomuki, &tree) {
            return t;
        }

        return if tree.has_lemma(vec!["?"]).is_some() {
            wakaran::new()
        } else {
            match omomuki {
                Omomuki::Dearu(dearu) => tawainai::dearu::new(&dearu),
                Omomuki::Shitai(shitai) => onegai::shitai::new(&shitai),
                Omomuki::Shite(shite) => onegai::shite::new(&shite),
                _ => Box::new(tawainai::sonota::Sonota {}),
            }
        };
    }
}

fn get_omomuki(tree: &cotoha::ParseObjects) -> Omomuki {
    // 動詞を探すべ
    if let Some((suru, shitai, shite)) = tree.get_doushi() {
        return if shitai {
            // フレンドしたい
            Omomuki::Shitai(suru)
        } else if shite {
            // 頑張って
            Omomuki::Shite(suru)
        } else {
            Omomuki::Suru(suru)
        };
    }

    // 次に体言止めを探すべ
    if let Some(taigen) = tree.get_taigen() {
        return Omomuki::Taigen(taigen);
    }
    // 形容詞語幹を探す
    if let Some(dou) = tree.get_keidou() {
        return Omomuki::Keiyou(dou);
    }
    // これはゾンビですか?
    if let Some(dearu) = tree.get_kore_are() {
        return Omomuki::Dearu(dearu);
    }

    // ばあちゃん、お茶
    let nani = tree.get_meishi();
    if nani.len() > 0 {
        return Omomuki::Ocha(Ocha {
            nani: nani,
            hatena: tree.is_hatena(),
        });
    }

    return Omomuki::Tawainai(Tawainai {
        itsu: None,
        doko: None,
        hatena: tree.is_hatena(),
    });
}
