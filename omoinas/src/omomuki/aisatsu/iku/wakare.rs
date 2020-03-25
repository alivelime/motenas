use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Wakare {}

impl Wakare {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai()
            && cotoha::has_lemma(chunks, vec!["さようなら", "さよなら"]).is_some()
        {
            return Some(Box::new(Wakare {}));
        }

        return None;
    }
}
impl Tumori for Wakare {
    fn get_kotae(&self) -> String {
        return String::from("はい、さようなら");
    }
}
