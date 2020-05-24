use crate::service::cotoha;
use crate::Tumori;

pub mod ittera;
pub mod matane;
pub mod sayonara;

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = matane::new(tree) {
        return Some(a);
    }
    if let Some(a) = sayonara::new(tree) {
        return Some(a);
    }

    return None;
}
