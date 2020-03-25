use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod hatsu;
pub mod modoru;
pub mod tama;
pub mod yoku;

pub fn new(
    omomuki: &omomuki::Omomuki,
    chunks: &Vec<cotoha::ParseObject>,
) -> Option<Box<dyn Tumori>> {
    if let Some(a) = hatsu::Hatsu::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = modoru::Modoru::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = yoku::Yoku::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = tama::Tama::new(omomuki, chunks) {
        return Some(a);
    }

    return None;
}
