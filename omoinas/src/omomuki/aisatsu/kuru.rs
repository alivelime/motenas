use crate::omomuki::Omomuki;
use crate::Tumori;

pub mod hajimemashite;
pub mod hisashiburi;
pub mod irasshai;
pub mod okaeri;
pub mod tadaima;

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = hajimemashite::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = tadaima::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = hisashiburi::new(omomuki) {
        return Some(a);
    }

    return None;
}
