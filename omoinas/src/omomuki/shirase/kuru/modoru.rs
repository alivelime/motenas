use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Modoru {}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    // 行ってきた
    if omomuki.dare == None
        && omomuki.doko == None
        && if let Some(d) = &omomuki.doushita {
            (d.suru == "戻る" || d.suru == "帰る") && d.toki == omomuki::Toki::Mukashi
        } else {
            false
        }
    {
        return Some(Box::new(crate::omomuki::aisatsu::kuru::okaeri::Okaeri {}));
    }

    return None;
}
