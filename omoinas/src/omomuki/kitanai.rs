use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Kitanai {
    ng: String,
}

impl Kitanai {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if let Some(ng) = cotoha::has_lemma(chunks, vec!["おい", "ババア", "ばばあ", "くそ"])
        {
            return Some(Box::new(Kitanai { ng: ng }));
        }

        return None;
    }
}

impl Tumori for Wakare {
    fn get_kotae(&self) -> String {
        return format!("{}とはなんだい!\nもっと綺麗な言葉をお使い!", self.ng);
    }
}
