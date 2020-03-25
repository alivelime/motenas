use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub mod aru;

pub fn new(
    omomuki: &omomuki::Omomuki,
    chunks: &Vec<cotoha::ParseObject>,
) -> Option<Box<dyn Tumori>> {
    if let Some(a) = aru::Aru::new(omomuki, chunks) {
        return Some(a);
    }

    return None;
}

