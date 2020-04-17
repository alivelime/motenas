use crate::hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Okaeri {}

impl Tumori for Okaeri {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.aisatsu.kuru.okaeri)());
    }
}
