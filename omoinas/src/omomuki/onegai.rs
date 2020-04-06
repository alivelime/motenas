pub mod motomeru;

use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub fn new(omomuki: &omomuki::Omomuki, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = motomeru::new(omomuki, tree) {
        return Some(a);
    }

    return None;
}
