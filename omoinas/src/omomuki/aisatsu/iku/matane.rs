use crate::cotoha;
use crate::Tumori;

pub struct Matane {
    pub itsu: Option<String>,
}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec!["また", "ばいばい", "バイバイ", "じゃあ", "さらば"])
        .is_some()
    {
        if let Some(itsu) = tree.get_itsu() {
            return Some(Box::new(Matane { itsu: Some(itsu) }));
        }
    }

    return None;
}

impl Tumori for Matane {
    fn get_kotae(&self) -> String {
        return if let Some(itsu) = &self.itsu {
            format!("はい、また{}", itsu)
        } else {
            String::from("また、いつでもおいで")
        };
    }
}
