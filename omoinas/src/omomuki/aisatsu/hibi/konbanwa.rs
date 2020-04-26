use chrono::{DateTime, Local, Timelike};

use crate::cotoha;
use crate::hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Konbanwa {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["こんばんは"]).is_some() {
        return Some(Box::new(Konbanwa {}));
    }
    return None;
}

impl Tumori for Konbanwa {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        let local: DateTime<Local> = Local::now();
        let time = local.hour();
        return Result::Message(match time {
            0..=3 => (chara.kaeshi.aisatsu.hibi.konbanwa.mayonaka)(),
            4..=9 => (chara.kaeshi.aisatsu.hibi.konbanwa.ohayo)(),
            10..=16 => (chara.kaeshi.aisatsu.hibi.konbanwa.konnichiwa)(),
            _ => (chara.kaeshi.aisatsu.hibi.konbanwa.konbanwa)(),
        });
    }
}