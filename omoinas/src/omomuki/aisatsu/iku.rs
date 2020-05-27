use crate::omomuki::Omomuki;
use crate::Tumori;

pub mod ittera;
pub mod matane;
pub mod sayonara;

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = matane::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = sayonara::new(omomuki) {
        return Some(a);
    }

    return None;
}
