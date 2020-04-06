use crate::cotoha;
use crate::Tumori;

pub mod aisatsu;
pub mod kitanai;
pub mod onegai;
pub mod shirase;
pub mod tawainai;
pub mod toikake;
// pub mod tsutae;
pub mod wakaran;

#[derive(Clone, Debug)]
pub enum HitoType {
    Wa, // 我
    Na, // 汝
    Ka, // 彼
}

/*
pub struct Hito {
    r#type: HitoType,
    name: String,
}
*/

#[derive(PartialOrd, PartialEq, Clone, Debug)]
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
pub struct Omomuki {
    itsu: Option<String>,
    doko: Option<String>,
    dare: Option<String>,
    nani: Option<Nani>,
    donoyouni: Option<String>,
    doushita: Option<Doushita>,
}

#[derive(Clone, Debug)]
pub struct Nani {
    donna: Option<String>,
    mono: String,
}

impl Omomuki {
    pub fn new(
        sentence_type: &cotoha::api::SentenceType,
        tree: &cotoha::ParseObjects,
    ) -> Box<dyn Tumori> {
        if let Some(a) = kitanai::new(&tree) {
            return a;
        }

        // まんず、あいさつかどうか調べるべ
        if let Some(a) = aisatsu::new(&tree) {
            return a;
        }

        let mut omomuki = Omomuki {
            itsu: None,
            doko: None,
            dare: None,
            nani: None,
            donoyouni: None,
            doushita: None,
        };

        // 次に、動詞を探すべ
        if let Some((suru, chunk_id, token_id)) = tree.get_doushi() {
            omomuki.doushita = Some(Doushita {
                ina: tree.is_shinai(chunk_id),
                ukemi: tree.is_ukemi(chunk_id),
                toki: if tree.is_mukashi(chunk_id) {
                    Toki::Mukashi
                } else {
                    Toki::Ima
                },
                suru: suru.clone(),
            });

            if let Some((nani, tid)) = tree.get_nani(token_id) {
                omomuki.nani = Some(Nani {
                    donna: tree.get_keiyou(tid),
                    mono: nani,
                });
            }
        } else if let Some((dou, _, token_id)) = tree.get_keidou() {
            // ない場合は形容詞語幹を探す
            omomuki.donoyouni = Some(dou);

            if let Some((nani, tid)) = tree.get_nani(token_id) {
                omomuki.nani = Some(Nani {
                    donna: tree.get_keiyou(tid),
                    mono: nani,
                });
            }
        }

        match sentence_type.modality.as_str() {
            "interrogative" => {
                // 問いかけ
                if let Some(t) = toikake::new(&omomuki, &tree) {
                    return t;
                }
            }
            "imperative" => {
                // お願い
                if let Some(t) = onegai::new(&omomuki, &tree) {
                    return t;
                }
            }
            _ => {
                // たわいない
                if omomuki.is_tawainai() {
                    if let Some(t) = tawainai::new(&tree) {
                        return t;
                    }
                    // とりあえず適当な奴には適当に返事する
                    return Box::new(tawainai::yobu::Yobu {});
                } else {
                    if let Some(t) = shirase::new(&omomuki, &tree) {
                        return t;
                    }
                    /*
                    if let Some(t) = tsutae::new(&omomuki, &tree) {
                        return t;
                    }
                    */
                }
            }
        };

        return wakaran::Wakaran::new();
    }

    pub fn is_tawainai(&self) -> bool {
        return self.itsu.is_none()
            && self.doko.is_none()
            && self.dare.is_none()
            && self.nani.is_none()
            && self.doushita.is_none();
    }
}
