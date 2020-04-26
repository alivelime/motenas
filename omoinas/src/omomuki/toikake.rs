use crate::omomuki;
use crate::Tumori;

pub mod aru;
pub mod desuka;

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = aru::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = desuka::new(omomuki) {
        return Some(a);
    }

    return None;
}
