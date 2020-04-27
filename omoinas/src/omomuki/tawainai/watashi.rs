use crate::omomuki;
use crate::Tumori;

pub mod ganbaru;
pub mod sukoyaka;
pub mod yamu;

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = sukoyaka::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = yamu::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = ganbaru::new(omomuki) {
        return Some(a);
    }

    return None;
}
