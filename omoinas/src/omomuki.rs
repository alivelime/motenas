use crate::model::kotoba::Koto;
use crate::model::mono::Mono;
use crate::model::omomuki::*;
use crate::model::parser::Parser;
use crate::Tumori;

pub mod aisatsu;
pub mod kitanai;
pub mod onegai;
pub mod shirase;
pub mod tawainai;
pub mod toikake;
pub mod wakaran;

#[derive(Clone, Debug)]
pub struct Omomuki {
    pub kotoba: Vec<Koto>,

    pub kitanai: Option<String>,
    pub yonda: bool,
    pub hatena: bool,

    pub odoroki: Option<Koto>,
    pub hitokoto: Option<Koto>,
    pub nakami: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
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
    pub fn new<P: Parser>(tree: &P) -> Omomuki {
        return Omomuki {
            kotoba: tree.get_kotoba(),

            kitanai: tree.is_kitanai(),
            yonda: tree.is_yonda(),
            hatena: tree.is_hatena(),

            odoroki: tree.get_odoroki(),
            hitokoto: tree.get_hitokoto(),
            nakami: Self::get_nakami(tree),
        };
    }
    fn get_nakami<P: Parser>(tree: &P) -> Type {
        // 動詞を探すべ
        if let Some((suru, shitai, shite)) = tree.get_doushi() {
            return if shitai {
                // フレンドしたい
                Type::Shitai(suru)
            } else if shite {
                // 頑張って
                Type::Shite(suru)
            } else {
                Type::Suru(suru)
            };
        }

        // 次に体言止めを探すべ
        if let Some(taigen) = tree.get_taigen() {
            return Type::Taigen(taigen);
        }
        // 形容詞語幹を探す
        if let Some(dou) = tree.get_keidou() {
            return Type::Keiyou(dou);
        }
        // これはゾンビですか?
        if let Some(dearu) = tree.get_kore_are() {
            return Type::Dearu(dearu);
        }

        // ばあちゃん、お茶
        let nani = tree.get_meishi();
        if nani.len() > 0 {
            return Type::Ocha(Ocha { nani: nani });
        }

        return Type::Tawainai(Tawainai {
            itsu: tree.get_itsu(),
            doko: tree.get_doko(),
        });
    }
    pub fn tumori(&self) -> Box<dyn Tumori> {
        // NGワードは弾く
        if let Some(a) = kitanai::new(&self) {
            return a;
        }

        // まんず、あいさつかどうか調べるべ
        if let Some(a) = aisatsu::new(&self) {
            return a;
        }

        // お願い
        if let Some(t) = onegai::new(&self) {
            return t;
        }

        // 問いかけ
        if let Some(t) = toikake::new(&self) {
            return t;
        }

        // 知らせる
        if let Some(t) = shirase::new(&self) {
            return t;
        }

        // たわいない
        if let Some(t) = tawainai::new(&self) {
            return t;
        }

        return if self.hatena {
            wakaran::new()
        } else {
            match &self.nakami {
                Type::Dearu(dearu) => tawainai::dearu::new(&dearu),
                Type::Shitai(shitai) => onegai::shitai::new(&shitai),
                Type::Shite(shite) => onegai::shite::new(&shite),
                _ => Box::new(tawainai::sonota::Sonota {}),
            }
        };
    }
    pub fn has(&self, p: Vec<&str>) -> Option<String> {
        return self.kotoba.iter().find_map(|k| {
            if p.contains(&k.moji.as_str()) {
                Some(k.moji.clone())
            } else {
                None
            }
        });
    }
    pub fn has_hitokoto(&self, p: Vec<&str>) -> Option<String> {
        return self
            .hitokoto
            .iter()
            .find_map(|k| match p.contains(&k.moji.as_str()) {
                true => Some(k.moji.clone()),
                false => None,
            });
    }
    pub fn has_odoroki(&self, p: Vec<&str>) -> Option<String> {
        return self
            .odoroki
            .iter()
            .find_map(|k| match p.contains(&k.moji.as_str()) {
                true => Some(k.moji.clone()),
                false => None,
            });
    }
}
