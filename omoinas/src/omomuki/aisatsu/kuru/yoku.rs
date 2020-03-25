use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Yoku {}

impl Yoku {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        _: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.dare.is_none()
            && if let Some(d) = &omomuki.doushita {
                d.suru == "来る" && d.toki == omomuki::Toki::Mukashi
            } else {
                false
            }
        {
            return Some(Box::new(Yoku {}));
        }
        if omomuki.dare.is_none()
            && omomuki.doko.is_none()
            && if let Some(d) = &omomuki.doushita {
                d.suru == "行く" && d.toki == omomuki::Toki::Mukashi
            } else {
                false
            }
        {
            return Some(Box::new(Yoku {}));
        }
        return None;
    }
}
impl Tumori for Yoku {
    fn get_kotae(&self) -> String {
        return String::from("はい、いらっしゃい");
    }
}
