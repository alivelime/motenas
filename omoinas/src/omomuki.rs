use log::debug;

use crate::cotoha;
use crate::Tumori;

pub mod aisatsu;
pub mod toikake;
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
    chunk_id: i32,
    token_id: i32,
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
    nani: Option<String>,
    donoyouni: Option<String>,
    doushita: Option<Doushita>,
}

impl Omomuki {
    pub fn new(
        sentence_type: &cotoha::SentenceType,
        objects: &cotoha::ParseObjects,
    ) -> Box<dyn Tumori> {
        let mut omomuki = Omomuki {
            itsu: None,
            doko: None,
            dare: None,
            nani: None,
            donoyouni: None,
            doushita: None,
        };

        // まんず、動詞を探すべ
        for chunk in &objects.chunks {
            for t in &chunk.tokens {
                if t.pos.as_str() == "動詞語幹" {
                    let mut doushita = Doushita {
                        chunk_id: chunk.chunk_info.id,
                        token_id: t.id,
                        ina: false,
                        ukemi: false,
                        toki: Toki::Ima,
                        suru: t.lemma.clone(),
                    };

                    // 時制と付加情報
                    match &chunk.chunk_info.predicate {
                        Some(predicate) => {
                            for p in predicate {
                                match p.as_str() {
                                    "negative" => doushita.ina = true,
                                    "past" => doushita.toki = Toki::Mukashi,
                                    "passive" => doushita.ukemi = true,
                                    _ => continue,
                                }
                            }
                        }
                        None => {}
                    };
                    omomuki.doushita = Some(doushita.clone());

                    // 何を
                    if let Some(labels) = &t.dependency_labels {
                        for dep in labels {
                            if objects.tokens[dep.token_id as usize].pos == "名詞"
                                && objects.tokens[dep.token_id as usize].lemma != "何か"
                                || dep.label == "nmod"
                            {
                                let nani = &objects.tokens[dep.token_id as usize];
                                omomuki.nani = Some(nani.lemma.clone());

                                if let Some(nlabels) = &nani.dependency_labels {
                                    for ndep in nlabels {
                                        if ndep.label == "amod" {
                                            omomuki.donoyouni = Some(
                                                objects.tokens[ndep.token_id as usize]
                                                    .lemma
                                                    .clone(),
                                            );
                                        }
                                    }
                                }

                                // あえてしない break;
                            }
                        }
                    }

                    // いつ・どこで
                    for link in &chunk.chunk_info.links {
                        match link.label.as_str() {
                            "time" => {
                                // いつ
                                omomuki.itsu =
                                    Some(cotoha::bunsetsu(&objects.chunks[link.link as usize]));
                            }
                            "purpose" => {
                                // 何を
                                omomuki.nani =
                                    Some(cotoha::bunsetsu(&objects.chunks[link.link as usize]));
                            }
                            _ => {}
                        }
                    }

                    break;
                }
                if let Some(features) = &t.features {
                    if features.contains(&String::from("日時")) && omomuki.itsu.is_none() {
                        omomuki.itsu = Some(t.lemma.clone());
                    }
                }
            }
        }
        debug!("{:#?}", omomuki);

        // あいさつ?
        if let Some(a) = aisatsu::new(&omomuki, &objects.chunks) {
            return a;
        }

        // 問いかけ? お願い?
        match sentence_type.modality.as_str() {
            "interrogative" => {
                // 問いかけ
                if let Some(t) = toikake::new(&omomuki, &objects.chunks) {
                    return t;
                }
            }
            "imperative" => {}
            _ => {}
        };

        return wakaran::Wakaran::new(&omomuki);
    }

    pub fn is_tawaimo_nai(&self) -> bool {
        return self.itsu.is_none()
            && self.doko.is_none()
            && self.dare.is_none()
            && self.nani.is_none()
            && self.doushita.is_none();
    }
}
