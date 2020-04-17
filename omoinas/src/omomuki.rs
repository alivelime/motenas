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
    nani: Option<Nani>,
    doushita: Doushita,
}

#[derive(Clone, Debug)]
pub struct Taigen {
    itsu: Option<String>,
    doko: Option<String>,
    nani: Option<Nani>,
    suru: String,
}

#[derive(Clone, Debug)]
pub struct Keiyou {
    itsu: Option<String>,
    doko: Option<String>,
    dare: Option<String>,
    nani: Option<Nani>,
    dou: String,
    ina: bool,
    toki: Toki,
}

#[derive(Clone, Debug)]
pub struct Dearu {}

#[derive(Clone, Debug)]
pub struct Ocha {
    nani: Vec<Nani>,
}

#[derive(Clone, Debug)]
pub struct Tawainai {
    itsu: Option<String>,
    doko: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Nani {
    pub donna: Option<String>,
    pub mono: Vec<String>,
}
impl Nani {
    pub fn donna_namae(&self) -> String {
        return format!(
            "{}{}",
            self.donna.as_ref().unwrap_or(&String::from("")),
            self.namae()
        );
    }
    pub fn namae(&self) -> String {
        return self
            .mono
            .iter()
            .rev()
            .fold(String::from(""), |s, m| format!("{}{}", s, m));
    }
}

pub enum Result {
    Message(String),
    Mono(String, Vec<model::mono::Mono>),
}

impl Omomuki {
    pub fn new(tree: &cotoha::ParseObjects) -> Box<dyn Tumori> {
        if let Some(a) = kitanai::new(&tree) {
            return a;
        }

        // まんず、あいさつかどうか調べるべ
        if let Some(a) = aisatsu::new(&tree) {
            return a;
        }

        let omomuki = get_omomuki(&tree);
        debug!("{:#?}", omomuki);

        // 問いかけ
        if tree.has_lemma(vec!["?"]).is_some() {
            if let Some(t) = toikake::new(&omomuki) {
                return t;
            }

            return wakaran::new();
        } else {
            // お願い
            if let Some(t) = onegai::new(&omomuki) {
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
        }

        return Box::new(tawainai::nani::Nani {});
    }
}

fn get_omomuki(tree: &cotoha::ParseObjects) -> Omomuki {
    // 動詞を探すべ
    if let Some((suru, chunk_id, token_id)) = tree.get_doushi() {
        return Omomuki::Suru(Suru {
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
                Some(Nani {
                    donna: tree.get_keiyou(tid),
                    mono: nani,
                })
            } else {
                None
            },
        });
    }

    // 次に体言止めを探すべ
    if let Some((suru, _chunk_id, token_id)) = tree.get_taigen() {
        return Omomuki::Taigen(Taigen {
            itsu: None,
            doko: None,
            suru: suru.clone(),
            nani: if let Some((nani, tid)) = tree.get_nani(token_id) {
                Some(Nani {
                    donna: tree.get_keiyou(tid),
                    mono: nani,
                })
            } else {
                None
            },
        });
    }
    if let Some((dou, chunk_id, token_id)) = tree.get_keidou() {
        // 形容詞語幹を探す
        return Omomuki::Keiyou(Keiyou {
            itsu: None,
            doko: None,
            dou: dou,
            nani: if let Some((nani, tid)) = tree.get_nani(token_id) {
                Some(Nani {
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
        });
    }

    let nani = tree.get_meishi();
    if nani.len() > 0 {
        return Omomuki::Ocha(Ocha {
            nani: nani
                .iter()
                .map(|(id, w)| Nani {
                    donna: tree.get_keiyou(*id),
                    mono: tree.add_compound(*id, vec![w.to_string()]),
                })
                .collect(),
        });
    }

    return Omomuki::Tawainai(Tawainai {
        itsu: tree.get_itsu(),
        doko: tree.get_doko(),
    });
}
