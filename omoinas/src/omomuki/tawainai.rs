use crate::service::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod arigato;
pub mod ayamaru;
pub mod bochibochi;
pub mod dearu;
pub mod douitashimashite;
pub mod hagemasu;
pub mod kizukai;
pub mod monku;
pub mod nani;
pub mod ocha;
pub mod sonota;
pub mod sounanda;
pub mod watashi;
pub mod yobu;
pub mod yokatta;

pub fn new(omomuki: &omomuki::Omomuki, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = arigato::new(tree) {
        return Some(a);
    }
    if let Some(a) = douitashimashite::new(tree) {
        return Some(a);
    }
    match omomuki {
        omomuki::Omomuki::Ocha(ocha) => {
            if let Some(a) = ocha::new(ocha, tree) {
                return Some(a);
            }
        }
        _ => {}
    };

    if let Some(a) = kizukai::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = hagemasu::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = watashi::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = monku::new(tree) {
        return Some(a);
    }
    if let Some(a) = yobu::new(tree) {
        return Some(a);
    }

    return None;
}
