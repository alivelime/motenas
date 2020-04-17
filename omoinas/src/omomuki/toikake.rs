use crate::omomuki;
use crate::Tumori;

pub mod aru;

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = aru::new(omomuki) {
        return Some(a);
    }

    return None;
}
