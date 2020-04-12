use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod modoru;
pub mod yoku;

pub fn new(omomuki: &omomuki::Suru, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = modoru::new(omomuki, tree) {
        return Some(a);
    }
    if let Some(a) = yoku::new(omomuki, tree) {
        return Some(a);
    }

    return None;
}
