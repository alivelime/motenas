pub mod api;

use crate::model::{Dearu, Doushita, Keiyou, Koto, Nani, Suru, Taigen, Toki};

pub struct ParseObjects {
    pub chunks: Vec<api::ParseObject>,
    pub tokens: Vec<api::Token>,
}

pub fn parse(token: &String, text: &String) -> Result<ParseObjects, reqwest::Error> {
    return match api::parse(token, text) {
        Ok((chunks, tokens)) => Ok(ParseObjects {
            chunks: chunks,
            tokens: tokens,
        }),
        Err(e) => Err(e),
    };
}

impl ParseObjects {
    pub fn has_lemma(&self, p: Vec<&str>) -> Option<String> {
        return self.tokens.iter().find_map(|t| {
            if p.contains(&t.lemma.as_str()) {
                Some(t.lemma.clone())
            } else {
                None
            }
        });
    }

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
    pub fn is_hatena(&self) -> bool {
        return self.has_lemma(vec!["?"]).is_some();
    }

    pub fn get_doushi(&self) -> Option<(Suru, bool, bool)> {
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
                            hatena: self.is_hatena(),
                        },
                        self.is_shitai(&t),
                        self.is_shite(&t),
                    ));
                }
            }
        }
        return None;
    }

    pub fn get_keidou(&self) -> Option<Keiyou> {
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
                        hatena: self.is_hatena(),
                    });
                }
            }
        }
        return None;
    }

    fn is_taigen(&self, t: &api::Token) -> bool {
        return t.pos.as_str() == "名詞"
            && t.features.iter().any(|f| f.contains(&String::from("動作")));
    }
    pub fn get_taigen(&self) -> Option<Taigen> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if self.is_taigen(t) {
                    return Some(Taigen {
                        itsu: None,
                        doko: None,
                        suru: Koto::new(&t.kana, &t.lemma),
                        nani: self.get_nani(t.id),
                        hatena: self.is_hatena(),
                    });
                }
            }
        }
        return None;
    }

    pub fn get_kore_are(&self) -> Option<Dearu> {
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
                        hatena: self.is_hatena(),
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
                    hatena: self.is_hatena(),
                });
            }
        }
        return None;
    }

    pub fn get_meishi(&self) -> Vec<Nani> {
        return self
            .chunks
            .iter()
            .flat_map(|c| c.tokens.iter())
            .filter(|t| t.pos.as_str() == "名詞" && self.is_main_meishi(t.id))
            .map(|t| Nani {
                donna: self.get_keiyou(t.id),
                mono: vec![Koto::new(&t.kana, &t.lemma)]
                    .into_iter()
                    .chain(self.get_compound(t.id).into_iter())
                    .collect::<Vec<Koto>>(),
            })
            .collect();
    }
    pub fn is_main_meishi(&self, id: i32) -> bool {
        return match &self.tokens.get((id + 1) as usize) {
            Some(t) => &t.pos != "名詞" || self.is_taigen(t),
            None => true,
        };
    }

    pub fn get_toki(&self, chunk_id: i32) -> Toki {
        return match self.is_mukashi(chunk_id) {
            true => Toki::Mukashi,
            false => Toki::Ima,
        };
    }
    pub fn is_mukashi(&self, chunk_id: i32) -> bool {
        return self.chunks[chunk_id as usize]
            .chunk_info
            .predicate
            .iter()
            .any(|ps| ps.contains(&String::from("past")));
    }
    pub fn is_nai(&self, chunk_id: i32) -> bool {
        return self.chunks[chunk_id as usize]
            .chunk_info
            .predicate
            .iter()
            .any(|ps| ps.contains(&String::from("negative")));
    }

    pub fn is_ukemi(&self, chunk_id: i32) -> bool {
        return self.chunks[chunk_id as usize]
            .chunk_info
            .predicate
            .iter()
            .any(|ps| ps.contains(&String::from("passive")));
    }

    pub fn get_nani(&self, tid: i32) -> Vec<Nani> {
        return self.tokens[tid as usize]
            .dependency_labels
            .iter()
            .flat_map(|dls| dls.iter())
            .filter_map(|dl| {
                let dep = &self.tokens[dl.token_id as usize];
                return if dep.pos == "名詞" && self.is_main_meishi(dep.id) {
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
    pub fn get_compound(&self, id: i32) -> Vec<Koto> {
        return self
            .tokens
            .iter()
            .take(id as usize)
            .rev()
            .take_while(|t| t.pos == "名詞")
            .map(|t| Koto::new(&t.kana, &t.lemma))
            .collect();
    }

    pub fn get_mokuteki(&self) -> Option<Koto> {
        return self
            .chunks
            .iter()
            .flat_map(|ch| ch.chunk_info.links.iter())
            .find_map(|link| match link.label.as_str() {
                "purpose" => Some(Koto::new("", self.bunsetsu(link.link as usize))),
                _ => None,
            });
    }
    pub fn get_itsu(&self) -> Option<Koto> {
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
    pub fn get_doko(&self) -> Option<Koto> {
        for t in self.chunks.iter().flat_map(|c| c.tokens.iter()) {
            if let Some(features) = &t.features {
                if features.contains(&String::from("固有:地")) {
                    return Some(Koto::new(&t.kana, &t.lemma));
                }
            }
        }
        return None;
    }

    pub fn get_keiyou(&self, tid: i32) -> Option<Koto> {
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
