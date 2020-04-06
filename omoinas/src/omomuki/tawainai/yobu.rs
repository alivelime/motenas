use rand::seq::SliceRandom;

use crate::cotoha;
use crate::Tumori;

pub struct Yobu {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec![
            "おーい",
            "ばあちゃん",
            "ばあさん",
            "やっほー",
            "やあ",
            "ねえねえ",
            "ねえ",
        ])
        .is_some()
    {
        return Some(Box::new(Yobu {}));
    }

    return None;
}

impl Tumori for Yobu {
    fn get_kotae(&self) -> String {
        return vec![
            "なんだい",
            "はいよ",
            "呼んだかい?",
            "どうかしたかい?",
            "そんなことより、お茶入れとくれ",
            "暖かくなってきたねぇ",
            "何か言ったかい?",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    }
}
