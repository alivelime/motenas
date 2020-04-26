pub mod motomeru;
pub mod shiritai;
pub mod shitai;
pub mod shite;

use crate::omomuki;
use crate::Tumori;

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = motomeru::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = shiritai::new(omomuki) {
        return Some(a);
    }

    return None;
}
