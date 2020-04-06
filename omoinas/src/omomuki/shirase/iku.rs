use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod dekakeru;
pub mod matakuru;

pub fn new(omomuki: &omomuki::Omomuki, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = dekakeru::new(omomuki, tree) {
        return Some(a);
    }
    if let Some(a) = matakuru::new(omomuki, tree) {
        return Some(a);
    }

    return None;
}
