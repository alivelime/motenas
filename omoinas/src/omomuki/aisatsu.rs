use crate::service::cotoha;
use crate::Tumori;

pub mod hibi;
pub mod iku;
pub mod kuru;

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = hibi::new(tree) {
        return Some(a);
    }
    if let Some(a) = iku::new(tree) {
        return Some(a);
    }
    if let Some(a) = kuru::new(tree) {
        return Some(a);
    }

    return None;
}
