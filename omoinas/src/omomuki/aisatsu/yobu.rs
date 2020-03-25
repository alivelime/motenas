use rand::seq::SliceRandom;

use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Yobu {}

impl Yobu {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai()
            && cotoha::has_lemma(
                chunks,
                vec![
                    "おーい",
                    "ばあちゃん",
                    "ばあさん",
                    "やっほー",
                    "やあ",
                    "ねえねえ",
                    "ねえ",
                ],
            )
            .is_some()
        {
            return Some(Box::new(Yobu {}));
        }

        return None;
    }
}
impl Tumori for Yobu {
    fn get_kotae(&self) -> String {
        return vec![
            "おや、なんだい",
            "はいよ",
            "呼んだかい?",
            "どうかしたかい?",
            "お茶が入ったよ",
            "暖かくなってきたねぇ",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    }
}
