use rand::seq::SliceRandom;

use crate::hitogata::*;

pub fn new() -> Hitogata {
    return Hitogata { kaeshi: &BACHAN };
}

pub const BACHAN: Kaeshi = Kaeshi {
    aisatsu: Aisatsu {
        hibi: AisatsuHibi {
            ohayo: || String::from("はい、おはよう"),
            konnichiwa: || String::from("はい、こんにちは!"),
            konbanwa: || String::from("はい、こんばんは"),
            oyasumi: || String::from("はい、おやすみ"),
        },
        iku: AisatsuIku {
            matane: AisatsuIkuMatane {
                toki: |itsu| format!("はい、また{}", itsu),
                tokoro: |doko| format!("はい、また{}で", doko),
                mata: || String::from("また、いつでもおいで"),
            },
            sayonara: || String::from("はい、さようなら"),
            ittera: AisatsuIkuIttera {
                toki: |itsu| format!("おや、{}に行くのかい。\nマスクは持ったかい?", itsu),
                tokoro: |doko| format!("おや、{}に行くのかい。\n人混みは避けるんだよ?", doko),
                tokitokoro: |itsu, doko| {
                    format!("おや、{}{}に行くのかい。\nマスクは持ったかい?", itsu, doko)
                },
                ittera: || String::from("はい、行ってらっしゃい\nマスクは持ったかい?"),
            },
        },
        kuru: AisatsuKuru {
            hajimemashite: || String::from("はい、はじめまして"),
            hisashiburi: || String::from("よう来たね"),
            okaeri: || String::from("はい、おかえり\n手洗いうがいしといで"),
            irasshai: || String::from("はい、いらっしゃい\n手をアルコール消毒しておくれ"),
        },
    },
    shirase: Shirase {
        iku: ShiraseIku {},
        kuru: ShiraseKuru {},
    },
    toikake: Toikake {
        aru: ToikakeAru {
            iroiro: |iroiro| format!("{}があるよ", iroiro.join("か")),
            aru: |mono| format!("はい、{}", mono),
            nai: |mono| format!("{}はないねぇ", mono),
            wakaran: |donna, mono| format!("{}{}はちょっとわからないねぇ", donna, mono),
            naikedo: |nai, aru| format!("{}はないけど{}ならあったかねぇ", nai, aru),
        },
    },
    tawainai: Tawainai {
        nani: || {
            vec![
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
            .to_string()
        },
    },
    wakaran: Wakaran {
        wakaran: || String::from("なんだい?"),
    },
    kitanai: |ng| format!("{}とはなんだい!\nもっと綺麗な言葉をお使い!", ng),
    error: Error {
        parse: || String::from("おや、解析APIでエラーかい"),
        sentence: || String::from("おや、文タイプ判別APIでエラーかい"),
        noimpl: || String::from("おや、未実装な反応だね"),
    },
};
