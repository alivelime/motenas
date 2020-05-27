use crate::omomuki::Omomuki;
use crate::Tumori;

pub mod hibi;
pub mod iku;
pub mod kuru;

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = hibi::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = iku::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = kuru::new(omomuki) {
        return Some(a);
    }

    return None;
}
