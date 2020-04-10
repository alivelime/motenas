use crate::hitogata;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Nani {}

impl Tumori for Nani {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.tawainai.nani)();
    }
}
