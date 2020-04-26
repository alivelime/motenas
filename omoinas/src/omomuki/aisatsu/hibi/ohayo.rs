use chrono::{DateTime, Local, Timelike};

use crate::cotoha;
use crate::hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ohayo {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec!["おはよう", "おはようございます"])
        .is_some()
    {
        return Some(Box::new(Ohayo {}));
    }
    return None;
}

impl Tumori for Ohayo {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        let local: DateTime<Local> = Local::now();
        let time = local.hour();
        return Result::Message(match time {
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
