use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Hiru {}

impl Hiru {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai() && cotoha::has_lemma(chunks, vec!["こんにちは"]).is_some()
        {
            return Some(Box::new(Hiru {}));
        }
        return None;
    }
}
impl Tumori for Hiru {
    fn get_kotae(&self) -> String {
        return String::from("はい、こんにちは");
    }
}
