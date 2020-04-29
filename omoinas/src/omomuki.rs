use log::debug;

use crate::cotoha;
use crate::model;
use crate::Tumori;

pub mod aisatsu;
pub mod kitanai;
pub mod onegai;
pub mod shirase;
pub mod tawainai;
pub mod toikake;
pub mod wakaran;

#[derive(PartialEq, Clone, Debug)]
pub enum Toki {
    Nochi,
    Ima,
    Mukashi,
}
#[derive(Clone, Debug)]
pub struct Doushita {
    ina: bool,
    toki: Toki,
    ukemi: bool,
    suru: String,
}
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

#[derive(Clone, Debug)]
pub struct Suru {
    itsu: Option<String>,
    doko: Option<String>,
    dare: Option<String>,
    nani: Option<model::Nani>,
    doushita: Doushita,
    hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Taigen {
    itsu: Option<String>,
    doko: Option<String>,
    nani: Option<model::Nani>,
    suru: String,
    hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Keiyou {
    itsu: Option<String>,
    doko: Option<String>,
    dare: Option<String>,
    nani: Option<model::Nani>,
    dou: String,
    ina: bool,
    toki: Toki,
    hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Dearu {
    kore: model::Nani,
    are: model::Nani,
    hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Ocha {
    nani: Vec<model::Nani>,
    hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Tawainai {
    itsu: Option<String>,
    doko: Option<String>,
    hatena: bool,
}

pub enum Result {
    Message(String),
    Mono(String, Vec<model::mono::Mono>),
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
    let hatena = tree.has_lemma(vec!["?"]).is_some();
    // 動詞を探すべ
    if let Some((suru, chunk_id, token_id, shitai, shite)) = tree.get_doushi() {
        let suru = Suru {
            itsu: None,
            doko: None,
            dare: None,
            doushita: Doushita {
                ina: tree.is_shinai(chunk_id),
                ukemi: tree.is_ukemi(chunk_id),
                toki: if tree.is_mukashi(chunk_id) {
                    Toki::Mukashi
                } else {
                    Toki::Ima
                },
                suru: suru.clone(),
            },
            nani: if let Some((nani, tid)) = tree.get_nani(token_id) {
                Some(model::Nani {
                    donna: tree.get_keiyou(tid),
                    mono: nani,
                })
            } else {
                None
            },
            hatena: hatena,
        };
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
    if let Some((suru, _chunk_id, token_id)) = tree.get_taigen() {
        return Omomuki::Taigen(Taigen {
            itsu: None,
            doko: None,
            suru: suru.clone(),
            nani: if let Some((nani, tid)) = tree.get_nani(token_id) {
                Some(model::Nani {
                    donna: tree.get_keiyou(tid),
                    mono: nani,
                })
            } else {
                None
            },
            hatena: hatena,
        });
    }
    // 形容詞語幹を探す
    if let Some((dou, chunk_id, token_id)) = tree.get_keidou() {
        return Omomuki::Keiyou(Keiyou {
            itsu: None,
            doko: None,
            dou: dou,
            nani: if let Some((nani, tid)) = tree.get_nani(token_id) {
                Some(model::Nani {
                    donna: tree.get_keiyou(tid),
                    mono: nani,
                })
            } else {
                None
            },
            dare: None,
            ina: tree.is_shinai(chunk_id),
            toki: if tree.is_mukashi(chunk_id) {
                Toki::Mukashi
            } else {
                Toki::Ima
            },
            hatena: hatena,
        });
    }
    // これはゾンビですか?
    if let Some((kore, ktid, are, ntid)) = tree.get_kore_nani() {
        return Omomuki::Dearu(Dearu {
            kore: model::Nani {
                donna: tree.get_keiyou(ktid),
                mono: tree.add_compound(ktid, vec![kore.clone()]),
            },
            are: model::Nani {
                donna: tree.get_keiyou(ntid),
                mono: tree.add_compound(ntid, vec![are.clone()]),
            },
            hatena: hatena,
        });
    }

    // ばあちゃん、お茶
    let nani = tree.get_meishi();
    if nani.len() > 0 {
        return Omomuki::Ocha(Ocha {
            nani: nani
                .iter()
                .map(|(id, w)| model::Nani {
                    donna: tree.get_keiyou(*id),
                    mono: tree.add_compound(*id, vec![w.to_string()]),
                })
                .collect(),
            hatena: hatena,
        });
    }

    return Omomuki::Tawainai(Tawainai {
        itsu: tree.get_itsu(),
        doko: tree.get_doko(),
        hatena: hatena,
    });
}
