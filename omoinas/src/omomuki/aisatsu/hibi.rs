use crate::service::cotoha;
use crate::Tumori;

pub mod konbanwa;
pub mod konnichiwa;
pub mod ohayo;
pub mod oyasumi;

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(a) = ohayo::new(tree) {
        return Some(a);
    }
    if let Some(a) = konnichiwa::new(tree) {
        return Some(a);
    }
    if let Some(a) = konbanwa::new(tree) {
        return Some(a);
    }
    if let Some(a) = oyasumi::new(tree) {
        return Some(a);
    }

    return None;
}
