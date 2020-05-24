use crate::service::cotoha;
use crate::Tumori;

pub mod hajimemashite;
pub mod hisashiburi;
pub mod irasshai;
pub mod okaeri;
pub mod tadaima;

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = hajimemashite::new(tree) {
        return Some(a);
    }
    if let Some(a) = tadaima::new(tree) {
        return Some(a);
    }
    if let Some(a) = hisashiburi::new(tree) {
        return Some(a);
    }

    return None;
}
