pub mod api;

use log::error;

use crate::model::cache::Cache;
use crate::model::kotoba::{Koto, Nani};
use crate::model::omomuki::{Dearu, Doushita, Keiyou, Suru, Taigen, Toki};
use crate::model::parser::Parser;

pub struct Cotoha {
    pub chunks: Vec<api::ParseObject>,
    pub tokens: Vec<api::Token>,
}

impl Parser for Cotoha {
    fn parse<C: Cache>(text: &String) -> (bool, Self) {
        let emp = (
            false,
            Cotoha {
                chunks: Vec::new(),
                tokens: Vec::new(),
            },
        );
        let token = match C::get_parser_token::<Self>() {
            Some(t) => t,
            None => {
                error!("error get parser token.");
                return emp;
            }
        };
        return match api::parse(&token, text) {
            Ok((chunks, tokens)) => (
                true,
                Cotoha {
                    chunks: chunks,
                    tokens: tokens,
                },
            ),
            Err(e) => {
                error!("{}", e);
                return emp;
            }
        };
    }
    fn get_access_token() -> Result<String, reqwest::Error> {
        return api::get_access_token();
    }

    fn is_hatena(&self) -> bool {
        return self.has_lemma(vec!["?"]).is_some();
    }
    fn is_yonda(&self) -> bool {
        return self
            .has_lemma(vec![
                "おーい",
                "ばあちゃん",
                "ばあさん",
                "やっほー",
                "やあ",
                "ねえねえ",
                "ねえ",
                "すみません",
            ])
            .is_some();
    }
    fn is_kitanai(&self) -> Option<String> {
        return self.has_lemma(vec![
            "おい",
            "ババア",
            "ばばあ",
            "くそ",
            "死ね",
            "馬鹿",
            "ふざける",
            "おっぱい",
            "パンツ",
        ]);
    }
    fn get_doushi(&self) -> Option<(Suru, bool, bool)> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if t.pos.as_str() == "動詞語幹" {
                    let chunk_id = chunk.chunk_info.id;
                    return Some((
                        Suru {
                            itsu: None,
                            doko: None,
                            dare: None,
                            doushita: Doushita {
                                ina: self.is_nai(chunk_id),
                                ukemi: self.is_ukemi(chunk_id),
                                toki: self.get_toki(chunk_id),
                                suru: Koto::new(&t.kana, &t.lemma),
                            },
                            nani: self.get_nani(t.id),
                        },
                        self.is_shitai(&t),
                        self.is_shite(&t),
                    ));
                }
            }
        }
        return None;
    }
    fn get_keidou(&self) -> Option<Keiyou> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if t.pos.as_str() == "形容詞語幹" {
                    let chunk_id = chunk.chunk_info.id;
                    return Some(Keiyou {
                        itsu: None,
                        doko: None,
                        dare: None,
                        dou: Koto::new(&t.kana, &t.lemma),
                        nani: self.get_nani(t.id),
                        ina: self.is_nai(chunk_id),
                        toki: self.get_toki(chunk_id),
                    });
                }
            }
        }
        return None;
    }
    fn get_taigen(&self) -> Option<Taigen> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if self.is_taigen(t) {
                    return Some(Taigen {
                        itsu: None,
                        doko: None,
                        suru: Koto::new(&t.kana, &t.lemma),
                        nani: self.get_nani(t.id),
                    });
                }
            }
        }
        return None;
    }

    fn get_kore_are(&self) -> Option<Dearu> {
        if self.tokens.iter().any(|t| {
            t.lemma == "判定詞"
                && t.features
                    .iter()
                    .any(|fs| fs.contains(&String::from("終止")))
        }) {
            // 猫
            if let Some(t) = self.tokens.iter().find(|t| {
                t.dependency_labels
                    .iter()
                    .flat_map(|dl| dl.iter())
                    .any(|dl| dl.label == "cop")
            }) {
                // 我輩
                if let Some(dep) = t
                    .dependency_labels
                    .iter()
                    .flat_map(|dl| dl.iter())
                    .find(|dl| dl.label == "nmod")
                {
                    let nt = &self.tokens[dep.token_id as usize];
                    return Some(Dearu {
                        kore: Nani {
                            donna: self.get_keiyou(t.id),
                            mono: vec![Koto::new(&t.kana, &t.lemma)]
                                .into_iter()
                                .chain(self.get_compound(t.id).into_iter())
                                .collect::<Vec<Koto>>(),
                        },
                        are: Nani {
                            donna: self.get_keiyou(nt.id),
                            mono: vec![Koto::new(&nt.kana, &nt.lemma)]
                                .into_iter()
                                .chain(self.get_compound(nt.id).into_iter())
                                .collect::<Vec<Koto>>(),
                        },
                    });
                }
            }
        }
        // 私ってホント馬鹿
        if let Some(t) = self.tokens.iter().find(|t| {
            t.dependency_labels
                .iter()
                .flat_map(|dl| dl.iter())
                .any(|dl| dl.label == "case")
        }) {
            if let Some(nt) = self.tokens.iter().find(|token| {
                token
                    .dependency_labels
                    .iter()
                    .flat_map(|dl| dl.iter())
                    .any(|dl| {
                        dl.token_id == t.id
                            && vec!["nsubj", "nmod", "csub", "nsub"].contains(&dl.label.as_str())
                    })
            }) {
                return Some(Dearu {
                    kore: Nani {
                        donna: self.get_keiyou(t.id),
                        mono: vec![Koto::new(&t.kana, &t.lemma)]
                            .into_iter()
                            .chain(self.get_compound(t.id).into_iter())
                            .collect::<Vec<Koto>>(),
                    },
                    are: Nani {
                        donna: self.get_keiyou(nt.id),
                        mono: vec![Koto::new(&nt.kana, &nt.lemma)]
                            .into_iter()
                            .chain(self.get_compound(nt.id).into_iter())
                            .collect::<Vec<Koto>>(),
                    },
                });
            }
        }
        return None;
    }
    fn get_meishi(&self) -> Vec<Nani> {
        return self
            .chunks
            .iter()
            .flat_map(|c| c.tokens.iter())
            .filter(|t| self.is_main_meishi(t))
            .map(|t| Nani {
                donna: self.get_keiyou(t.id),
                mono: vec![Koto::new(&t.kana, &t.lemma)]
                    .into_iter()
                    .chain(self.get_compound(t.id).into_iter())
                    .collect::<Vec<Koto>>(),
            })
            .collect();
    }
    fn get_kotoba(&self) -> Vec<Koto> {
        return self
            .tokens
            .iter()
            .map(|t| Koto {
                yomi: t.kana.clone(),
                moji: t.lemma.clone(),
            })
            .collect::<Vec<Koto>>();
    }
    fn get_odoroki(&self) -> Option<Koto> {
        return self.tokens.iter().find_map(|t| match t.pos.as_str() {
            "感嘆詞" => Some(Koto {
                yomi: t.kana.clone(),
                moji: t.lemma.clone(),
            }),
            _ => None,
        });
    }
    fn get_hitokoto(&self) -> Option<Koto> {
        return self.tokens.iter().find_map(|t| match t.pos.as_str() {
            "独立詞" => Some(Koto {
                yomi: t.kana.clone(),
                moji: t.lemma.clone(),
            }),
            _ => None,
        });
    }
    fn get_itsu(&self) -> Option<Koto> {
        for chunk in &self.chunks {
            for link in &chunk.chunk_info.links {
                match link.label.as_str() {
                    "time" => {
                        return Some(Koto::new("", self.bunsetsu(link.link as usize)));
                    }
                    _ => {}
                }
            }

            for t in &chunk.tokens {
                if let Some(features) = &t.features {
                    if features.contains(&String::from("日時")) {
                        return Some(Koto::new(&t.kana, &t.lemma));
                    }
                }
            }
        }
        return None;
    }
    fn get_doko(&self) -> Option<Koto> {
        for t in self.chunks.iter().flat_map(|c| c.tokens.iter()) {
            if let Some(features) = &t.features {
                if features.contains(&String::from("固有:地")) {
                    return Some(Koto::new(&t.kana, &t.lemma));
                }
            }
        }
        return None;
    }

    fn has_lemma(&self, p: Vec<&str>) -> Option<String> {
        return self
            .tokens
            .iter()
            .find_map(|t| match p.contains(&t.lemma.as_str()) {
                true => Some(t.lemma.clone()),
                false => None,
            });
    }
}

impl Cotoha {
    fn is_shitai(&self, t: &api::Token) -> bool {
        return t
            .dependency_labels
            .iter()
            .flat_map(|dls| dls.iter())
            .any(|dl| {
                dl.label == "aux"
                    && self.tokens[dl.token_id as usize].lemma == "たい"
                    && self.tokens[dl.token_id as usize]
                        .features
                        .iter()
                        .flat_map(|fs| fs.iter())
                        .any(|f| vec!["終止", "形容詞語幹"].contains(&f.as_str()))
            });
    }
    fn is_shite(&self, t: &api::Token) -> bool {
        return t
            .dependency_labels
            .iter()
            .flat_map(|dls| dls.iter())
            .any(|dl| {
                dl.label == "aux"
                    && self.tokens[dl.token_id as usize]
                        .features
                        .iter()
                        .any(|fs| fs.contains(&String::from("命令")))
            });
    }

    fn is_taigen(&self, t: &api::Token) -> bool {
        return t.pos.as_str() == "名詞"
            && t.features.iter().any(|f| {
                f.contains(&String::from("動作"))
                    && match &self.tokens.get((t.id + 1) as usize) {
                        Some(n) => n.pos != "名詞",
                        None => true,
                    }
            });
    }
    fn is_main_meishi(&self, dep: &api::Token) -> bool {
        return self.is_meishi(dep)
            && match &self.tokens.get((dep.id + 1) as usize) {
                Some(t) => &t.pos != "名詞" || self.is_taigen(t),
                None => true,
            };
    }
    fn is_meishi(&self, t: &api::Token) -> bool {
        return t.pos == "名詞" || t.features.iter().any(|f| f.contains(&String::from("名詞")));
    }

    fn get_toki(&self, chunk_id: i32) -> Toki {
        return match self.is_mukashi(chunk_id) {
            true => Toki::Mukashi,
            false => Toki::Ima,
        };
    }
    fn is_mukashi(&self, chunk_id: i32) -> bool {
        return self.chunks[chunk_id as usize]
            .chunk_info
            .predicate
            .iter()
            .any(|ps| ps.contains(&String::from("past")));
    }
    fn is_nai(&self, chunk_id: i32) -> bool {
        return self.chunks[chunk_id as usize]
            .chunk_info
            .predicate
            .iter()
            .any(|ps| ps.contains(&String::from("negative")));
    }

    fn is_ukemi(&self, chunk_id: i32) -> bool {
        return self.chunks[chunk_id as usize]
            .chunk_info
            .predicate
            .iter()
            .any(|ps| ps.contains(&String::from("passive")));
    }

    fn get_nani(&self, tid: i32) -> Vec<Nani> {
        return self.tokens[tid as usize]
            .dependency_labels
            .iter()
            .flat_map(|dls| dls.iter())
            .filter_map(|dl| {
                let dep = &self.tokens[dl.token_id as usize];
                return if self.is_main_meishi(dep) {
                    Some(Nani {
                        donna: self.get_keiyou(dep.id),
                        mono: vec![Koto::new(&dep.kana, &dep.lemma)]
                            .into_iter()
                            .chain(self.get_compound(dep.id).into_iter())
                            .collect::<Vec<Koto>>(),
                    })
                } else {
                    None
                };
            })
            .collect::<Vec<Nani>>();
    }
    fn get_compound(&self, id: i32) -> Vec<Koto> {
        return self
            .tokens
            .iter()
            .take(id as usize)
            .rev()
            .take_while(|t| t.pos == "名詞")
            .map(|t| Koto::new(&t.kana, &t.lemma))
            .collect();
    }

    /*
    fn get_mokuteki(&self) -> Option<Koto> {
        return self
            .chunks
            .iter()
            .flat_map(|ch| ch.chunk_info.links.iter())
            .find_map(|link| match link.label.as_str() {
                "purpose" => Some(Koto::new("", self.bunsetsu(link.link as usize))),
                _ => None,
            });
    }
    */

    fn get_keiyou(&self, tid: i32) -> Option<Koto> {
        return self.tokens[tid as usize]
            .dependency_labels
            .iter()
            .flat_map(|dls| dls.iter())
            .find_map(|dl| {
                let dep = &self.tokens[dl.token_id as usize];
                return match dl.label.as_str() {
                    "amod" => Some(Koto::new(&dep.kana, &dep.lemma)),
                    _ => None,
                };
            });
    }
    fn bunsetsu(&self, chunk_id: usize) -> String {
        let mut s = String::new();
        for t in &self.chunks[chunk_id].tokens {
            if api::imiaru(t.pos.as_str()) {
                s = format!("{}{}", s, t.form);
            }
        }
        return s;
    }
}

/*
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}
*/
