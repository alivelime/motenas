use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none() {
        if if let Some(d) = &omomuki.doushita {
            d.suru == "来る" && d.toki == omomuki::Toki::Ima
        } else {
            false
        } {
            return Some(Box::new(crate::omomuki::aisatsu::iku::matane::Matane {
                itsu: None,
            }));
        }
    }

    return None;
}
