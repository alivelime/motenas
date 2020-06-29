use crate::omomuki::Omomuki;
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

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = arigato::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = douitashimashite::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = kizukai::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = hagemasu::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = watashi::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = monku::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = yobu::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = ocha::new(omomuki) {
        return Some(a);
    }

    return None;
}
