pub mod motomeru;

use crate::omomuki;
use crate::Tumori;

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = motomeru::new(omomuki) {
        return Some(a);
    }

    return None;
}
