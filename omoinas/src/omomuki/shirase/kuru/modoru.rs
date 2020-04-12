use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Modoru {}

pub fn new(omomuki: &omomuki::Suru, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    // 行ってきた
    if omomuki.dare == None
        && (omomuki.doushita.suru == "戻る" || omomuki.doushita.suru == "帰る")
        && omomuki.doushita.toki == omomuki::Toki::Mukashi
    {
        return Some(Box::new(Modoru {}));
    }

    return None;
}

impl Tumori for Modoru {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::kuru::okaeri::Okaeri {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.error.noimpl)();
    }
}
