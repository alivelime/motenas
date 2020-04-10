use crate::cotoha;
use crate::Tumori;

pub mod nani;
pub mod yobu;

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = yobu::new(tree) {
        return Some(a);
    }

    return None;
}
