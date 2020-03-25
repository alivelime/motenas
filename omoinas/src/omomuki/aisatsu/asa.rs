use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Asa {}

impl Asa {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai()
            && cotoha::has_lemma(chunks, vec!["おはよう", "おはようございます"]).is_some()
        {
            return Some(Box::new(Asa {}));
        }
        return None;
    }
}
impl Tumori for Asa {
    fn get_kotae(&self) -> String {
        return String::from("はい、おはよう");
    }
}
