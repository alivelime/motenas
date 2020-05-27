use chrono::{FixedOffset, Timelike, Utc};

use crate::model::hitogata::Hitogata;
use crate::omomuki::{Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ohayo {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if omomuki
        .has_hitokoto(vec!["おはよう", "おはようございます"])
        .is_some()
    {
        return Some(Box::new(Ohayo {}));
    }
    return None;
}

impl Tumori for Ohayo {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        let local = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
        return Result::Message(match local.hour() {
            0..=3 => (chara.kaeshi.aisatsu.hibi.ohayo.mayonaka)(),
            4..=6 => (chara.kaeshi.aisatsu.hibi.ohayo.akegata)(),
            7..=9 => (chara.kaeshi.aisatsu.hibi.ohayo.ohayo)(),
            10..=12 => (chara.kaeshi.aisatsu.hibi.ohayo.osoyo)(),
            13..=16 => (chara.kaeshi.aisatsu.hibi.ohayo.ohiru)(),
            17..=19 => (chara.kaeshi.aisatsu.hibi.ohayo.kure)(),
            _ => (chara.kaeshi.aisatsu.hibi.ohayo.yoru)(),
        });
    }
}
