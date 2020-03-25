use crate::omomuki;
use crate::Tumori;

pub struct Wakaran {
    omomuki: omomuki::Omomuki,
}

impl Wakaran {
    pub fn new(omomuki: &omomuki::Omomuki) -> Box<dyn Tumori> {
        return Box::new(Wakaran {
            omomuki: omomuki.clone(),
        });
    }
}
impl Tumori for Wakaran {
    fn get_kotae(&self) -> String {
        return String::from("ちょっとわかんないねぇ");
    }
}
