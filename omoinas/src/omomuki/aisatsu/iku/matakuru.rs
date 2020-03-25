use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Matakuru {
    omomuki: omomuki::Omomuki,
}

impl Matakuru {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.dare.is_none() && omomuki.doko.is_none() && omomuki.nani.is_none() {
            if if let Some(d) = &omomuki.doushita {
                d.suru == "来る" && d.toki == omomuki::Toki::Ima
            } else {
                false
            } {
                return Some(Box::new(Matakuru {
                    omomuki: omomuki.clone(),
                }));
            }

            if cotoha::has_lemma(chunks, vec!["また", "ばいばい", "バイバイ", "じゃあ"]).is_some()
            {
                return Some(Box::new(Matakuru {
                    omomuki: omomuki.clone(),
                }));
            }
        }

        return None;
    }
}
impl Tumori for Matakuru {
    fn get_kotae(&self) -> String {
        return if let Some(itsu) = &self.omomuki.itsu {
            format!("はい、また{}", itsu)
        } else {
            String::from("また、いつでもおいで")
        };
    }
}
