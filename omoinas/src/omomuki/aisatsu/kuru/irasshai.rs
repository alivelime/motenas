use crate::hitogata;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Irasshai {}

impl Tumori for Irasshai {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.aisatsu.kuru.irasshai)();
    }
}
