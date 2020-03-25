use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod asa;
pub mod hiru;
pub mod iku;
pub mod kuru;
pub mod neru;
pub mod yobu;
pub mod yoru;

pub fn new(
    omomuki: &omomuki::Omomuki,
    chunks: &Vec<cotoha::ParseObject>,
) -> Option<Box<dyn Tumori>> {
    if let Some(a) = asa::Asa::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = hiru::Hiru::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = yoru::Yoru::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = neru::Neru::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = kuru::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = iku::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = yobu::Yobu::new(omomuki, chunks) {
        return Some(a);
    }

    return None;
}
