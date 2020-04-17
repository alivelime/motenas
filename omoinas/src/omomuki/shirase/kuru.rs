use crate::omomuki;
use crate::Tumori;

pub mod modoru;
pub mod yoku;

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    if let Some(a) = modoru::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = yoku::new(omomuki) {
        return Some(a);
    }

    return None;
}
