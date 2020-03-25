use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod dekakeru;
pub mod matakuru;
pub mod wakare;

pub fn new(
    omomuki: &omomuki::Omomuki,
    chunks: &Vec<cotoha::ParseObject>,
) -> Option<Box<dyn Tumori>> {
    if let Some(a) = dekakeru::Dekakeru::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = matakuru::Matakuru::new(omomuki, chunks) {
        return Some(a);
    }
    if let Some(a) = wakare::Wakare::new(omomuki, chunks) {
        return Some(a);
    }

    return None;
}
