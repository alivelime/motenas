use crate::omomuki;
use crate::Tumori;

pub mod dekakeru;
pub mod matakuru;

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    if let Some(a) = dekakeru::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = matakuru::new(omomuki) {
        return Some(a);
    }

    return None;
}
