use crate::omomuki::Omomuki;
use crate::Tumori;

pub mod konbanwa;
pub mod konnichiwa;
pub mod ohayo;
pub mod oyasumi;

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(a) = ohayo::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = konnichiwa::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = konbanwa::new(omomuki) {
        return Some(a);
    }
    if let Some(a) = oyasumi::new(omomuki) {
        return Some(a);
    }

    return None;
}
