use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Hatsu {}

impl Hatsu {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai()
            && cotoha::has_lemma(chunks, vec!["はじめまして", "初めまして"]).is_some()
        {
            return Some(Box::new(Hatsu {}));
        }
        return None;
    }
}
impl Tumori for Hatsu {
    fn get_kotae(&self) -> String {
        return String::from("はい、ご丁寧にどうも");
    }
}
