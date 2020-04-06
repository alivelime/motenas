use crate::Tumori;

pub struct Wakaran {}

impl Wakaran {
    pub fn new() -> Box<dyn Tumori> {
        return Box::new(Wakaran {});
    }
}
impl Tumori for Wakaran {
    fn get_kotae(&self) -> String {
        return String::from("ちょっとわかんないねぇ");
    }
}
