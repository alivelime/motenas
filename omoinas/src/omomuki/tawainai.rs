use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod nani;
pub mod ocha;
pub mod yobu;

pub fn new(omomuki: &omomuki::Omomuki, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    return match omomuki {
        omomuki::Omomuki::Ocha(ocha) => {
            if let Some(a) = ocha::new(ocha, tree) {
                Some(a)
            } else {
                None
            }
        }
        _ => {
            if let Some(a) = yobu::new(tree) {
                Some(a)
            } else {
                None
            }
        }
    };
}
