use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod arigato;
pub mod ayamaru;
pub mod bochibochi;
pub mod dearu;
pub mod hagemasu;
pub mod kizukai;
pub mod monku;
pub mod nani;
pub mod ocha;
pub mod sonota;
pub mod sounanda;
pub mod yobu;

pub fn new(omomuki: &omomuki::Omomuki, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Ocha(ocha) => {
            if let Some(a) = ocha::new(ocha, tree) {
                return Some(a);
            }
        }
        _ => {
            if let Some(a) = hagemasu::new(omomuki) {
                return Some(a);
            }
            if let Some(a) = monku::new(tree) {
                return Some(a);
            }
            if let Some(a) = yobu::new(tree) {
                return Some(a);
            }
        }
    };

    return None;
}
