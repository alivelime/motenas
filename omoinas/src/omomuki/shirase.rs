use crate::omomuki;
use crate::Tumori;

pub mod iku;
pub mod kuru;
pub mod neru;

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    return match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if let Some(a) = iku::new(&suru) {
                Some(a)
            } else if let Some(a) = kuru::new(&suru) {
                Some(a)
            } else if let Some(a) = neru::new(&suru) {
                Some(a)
            } else {
                None
            }
        }
        _ => None,
    };
}
