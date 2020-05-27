use chrono::{FixedOffset, Timelike, Utc};

use crate::model::hitogata::Hitogata;
use crate::omomuki::{Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Oyasumi {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if omomuki
        .has_hitokoto(vec!["おやすみ", "おやすみなさい"])
        .is_some()
    {
        return Some(Box::new(Oyasumi {}));
    }

    return None;
}

impl Tumori for Oyasumi {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        let local = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
        return Result::Message(match local.hour() {
            0..=3 => (chara.kaeshi.aisatsu.hibi.oyasumi.mayonaka)(),
            4..=9 => (chara.kaeshi.aisatsu.hibi.oyasumi.ohayo)(),
            10..=16 => (chara.kaeshi.aisatsu.hibi.oyasumi.konnichiwa)(),
            _ => (chara.kaeshi.aisatsu.hibi.oyasumi.oyasumi)(),
        });
    }
}
