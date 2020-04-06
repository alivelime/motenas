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

    pub fn get_doushi(&self) -> Option<(String, i32, i32)> {
        for chunk in &self.chunks {
            for t in &chunk.tokens {
                if t.pos.as_str() == "動詞語幹" {
                    return Some((t.lemma.clone(), chunk.chunk_info.id, t.id));
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

    pub fn get_nani(&self, t: i32) -> Option<(String, i32)> {
        let mut nani = String::from("");
        let mut tid: i32 = 0;
        if let Some(labels) = &self.tokens[t as usize].dependency_labels {
            for dep in labels {
                if dep.token_id < t
                    && self.tokens[dep.token_id as usize].pos == "名詞"
                    && self.tokens[dep.token_id as usize].lemma != "何か"
                {
                    nani = self.tokens[dep.token_id as usize].lemma.clone();
                    tid = dep.token_id;
                }
            }
        }
        return Some((nani, tid));
    }
    pub fn get_itsu(&self) -> Option<String> {
        for chunk in &self.chunks {
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
}
/*
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

   if let Some(features) = &t.features {
       if features.contains(&String::from("日時")) && omomuki.itsu.is_none() {
           omomuki.itsu = Some(t.lemma.clone());
       }
   }
*
*/

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
