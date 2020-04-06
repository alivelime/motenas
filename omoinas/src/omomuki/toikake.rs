use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod aru;

pub fn new(omomuki: &omomuki::Omomuki, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = aru::new(omomuki, tree) {
        return Some(a);
    }

    return None;
}
