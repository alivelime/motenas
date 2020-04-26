pub mod api;

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
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if p.contains(&t.lemma.as_str()) {
                    return Some(t.lemma.clone());
                }
            }
        }
        return None;
    }

    pub fn get_doushi(&self) -> Option<(String, i32, i32, bool, bool)> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if t.pos.as_str() == "動詞語幹" {
                    return Some((
                        t.lemma.clone(),
                        chunk.chunk_info.id,
                        t.id,
                        t.dependency_labels
                            .iter()
                            .flat_map(|dl| dl.iter())
                            .any(|dl| {
                                dl.label == "aux"
                                    && self.tokens[dl.token_id as usize].lemma == "たい"
                                    && self.tokens[dl.token_id as usize]
                                        .features
                                        .iter()
                                        .flat_map(|f| f.iter())
                                        .any(|f| f.as_str() == "終止")
                            }),
                        t.dependency_labels
                            .iter()
                            .flat_map(|dl| dl.iter())
                            .any(|dl| {
                                dl.label == "aux"
                                    && self.tokens[dl.token_id as usize]
                                        .features
                                        .iter()
                                        .flat_map(|f| f.iter())
                                        .any(|f| f.as_str() == "命令")
                            }),
                    ));
                }
            }
        }
        return None;
    }

    pub fn get_keidou(&self) -> Option<(String, i32, i32)> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if t.pos.as_str() == "形容詞語幹" {
                    return Some((t.lemma.clone(), chunk.chunk_info.id, t.id));
                }
            }
        }
        return None;
    }

    pub fn get_taigen(&self) -> Option<(String, i32, i32)> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if t.pos.as_str() == "名詞"
                    && if let Some(f) = &t.features {
                        f.contains(&String::from("動作"))
                    } else {
                        false
                    }
                {
                    return Some((t.lemma.clone(), chunk.chunk_info.id, t.id));
                }
            }
        }
        return None;
    }

    pub fn get_kore_nani(&self) -> Option<(String, i32, String, i32)> {
        if self.tokens.iter().any(|t| {
            t.lemma == "判定詞"
                && match &t.features {
                    Some(f) => f.contains(&String::from("終止")),
                    None => false,
                }
        }) {
            if let Some(t) = self.tokens.iter().find(|t| {
                t.dependency_labels
                    .iter()
                    .flat_map(|dl| dl.iter())
                    .any(|dl| dl.label == "cop")
            }) {
                if let Some(nt) = t
                    .dependency_labels
                    .iter()
                    .flat_map(|dl| dl.iter())
                    .find(|dl| dl.label == "nmod")
                {
                    return Some((
                        t.lemma.clone(),
                        t.id,
                        self.tokens[nt.token_id as usize].lemma.clone(),
                        nt.token_id,
                    ));
                }
            }
        } else {
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
                                && vec!["nmod", "csub", "nsub"].contains(&dl.label.as_str())
                        })
                }) {
                    return Some((t.lemma.clone(), t.id, nt.lemma.clone(), nt.id));
                }
            }
        };
        return None;
    }

    pub fn get_meishi(&self) -> Vec<(i32, String)> {
        return self
            .chunks
            .iter()
            .flat_map(|c| c.tokens.iter())
            .filter(|t| t.pos.as_str() == "名詞" && self.not_compound(t.id))
            .map(|t| (t.id, t.lemma.clone()))
            .collect();
    }
    pub fn not_compound(&self, id: i32) -> bool {
        return self
            .tokens
            .iter()
            .flat_map(|t| t.dependency_labels.iter())
            .flat_map(|dl| dl.iter())
            .all(|l| !(l.token_id == id && l.label == "compound"));
    }

    pub fn is_mukashi(&self, chunk_id: i32) -> bool {
        if let Some(p) = &self.chunks[chunk_id as usize].chunk_info.predicate {
            if p.contains(&String::from("past")) {
                return true;
            }
        }
        return false;
    }
    pub fn is_shinai(&self, chunk_id: i32) -> bool {
        if let Some(p) = &self.chunks[chunk_id as usize].chunk_info.predicate {
            if p.contains(&String::from("negative")) {
                return true;
            }
        }
        return false;
    }

    pub fn is_ukemi(&self, chunk_id: i32) -> bool {
        if let Some(p) = &self.chunks[chunk_id as usize].chunk_info.predicate {
            if p.contains(&String::from("passive")) {
                return true;
            }
        }
        return false;
    }

    pub fn get_nani(&self, t: i32) -> Option<(Vec<String>, i32)> {
        let mut nani: Vec<String> = Vec::new();
        let mut tid: i32 = -1;
        for dl in self.tokens[t as usize]
            .dependency_labels
            .iter()
            .flat_map(|d| d.iter())
        {
            let dep = &self.tokens[dl.token_id as usize];
            if dl.token_id < t && dep.pos == "名詞" {
                tid = dl.token_id;
                match dep.lemma.as_str() {
                    "何" | "何か" | "ばあちゃん" => continue,
                    "物" => nani.push(String::from("モノ")),
                    _ => {
                        nani.push(dep.lemma.clone());
                        nani = self.add_compound(tid, nani);
                    }
                }
            }
        }
        return if tid != -1 { Some((nani, tid)) } else { None };
    }
    pub fn add_compound(&self, id: i32, mut v: Vec<String>) -> Vec<String> {
        for dl in self.tokens[id as usize]
            .dependency_labels
            .iter()
            .flat_map(|dl| dl.iter())
        {
            if dl.label.as_str() == "compound" && self.tokens[dl.token_id as usize].pos == "名詞"
            {
                v.push(self.tokens[dl.token_id as usize].lemma.clone());
            }
        }
        return v;
    }

    pub fn get_mokuteki(&self) -> Option<String> {
        for chunk in &self.chunks {
            for link in &chunk.chunk_info.links {
                match link.label.as_str() {
                    "purpose" => {
                        // 何を
                        return Some(self.bunsetsu(link.link as usize));
                    }
                    _ => {}
                }
            }
        }
        return None;
    }
    pub fn get_itsu(&self) -> Option<String> {
        for chunk in &self.chunks {
            for link in &chunk.chunk_info.links {
                match link.label.as_str() {
                    "time" => {
                        return Some(self.bunsetsu(link.link as usize));
                    }
                    _ => {}
                }
            }

            for t in &chunk.tokens {
                if let Some(features) = &t.features {
                    if features.contains(&String::from("日時")) {
                        return Some(t.lemma.clone());
                    }
                }
            }
        }
        return None;
    }
    pub fn get_doko(&self) -> Option<String> {
        for t in self.chunks.iter().flat_map(|c| c.tokens.iter()) {
            if let Some(features) = &t.features {
                if features.contains(&String::from("固有:地")) {
                    return Some(t.lemma.clone());
                }
            }
        }
        return None;
    }

    pub fn get_keiyou(&self, tid: i32) -> Option<String> {
        if let Some(deps) = &self.tokens[tid as usize].dependency_labels {
            for dep in deps {
                if dep.label == "amod" {
                    return Some(self.tokens[dep.token_id as usize].lemma.clone());
                }
            }
        }
        return None;
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
