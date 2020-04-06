use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod iku;
pub mod kuru;
pub mod neru;

pub fn new(omomuki: &omomuki::Omomuki, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = iku::new(omomuki, tree) {
        return Some(a);
    }
    if let Some(a) = kuru::new(omomuki, tree) {
        return Some(a);
    }
    if let Some(a) = neru::new(omomuki, tree) {
        return Some(a);
    }

    return None;
}
