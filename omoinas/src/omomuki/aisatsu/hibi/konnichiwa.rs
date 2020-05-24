use chrono::{FixedOffset, Timelike, Utc};

use crate::service::cotoha;
use crate::model::hitogata::Hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Konnichiwa {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["こんにちは"]).is_some() {
        return Some(Box::new(Konnichiwa {}));
    }
    return None;
}

impl Tumori for Konnichiwa {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        let local = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
        return Result::Message(match local.hour() {
            0..=3 => (chara.kaeshi.aisatsu.hibi.konnichiwa.mayonaka)(),
            4..=9 => (chara.kaeshi.aisatsu.hibi.konnichiwa.ohayo)(),
            10..=16 => (chara.kaeshi.aisatsu.hibi.konnichiwa.konnichiwa)(),
            _ => (chara.kaeshi.aisatsu.hibi.konnichiwa.konbanwa)(),
        });
    }
}
