use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Tama {}

impl Tama {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai()
            && cotoha::has_lemma(chunks, vec!["久しぶり", "ひさしぶり"]).is_some()
        {
            return Some(Box::new(Tama {}));
        }
        return None;
    }
}
impl Tumori for Tama {
    fn get_kotae(&self) -> String {
        return String::from("よう来たね");
    }
}
