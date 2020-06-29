use rand::seq::SliceRandom;

use crate::model::hitogata::*;

pub const BACHAN: Kaeshi = Kaeshi {
    aisatsu: Aisatsu {
        hibi: AisatsuHibi {
            ohayo: AisatsuHibiOhayo {
                mayonaka: || String::from("まだ夜中だよ? 夜勤かい?"),
                akegata: || String::from("おはよう。早いんだねぇ"),
                ohayo: || String::from("はい、おはよう"),
                osoyo: || String::from("やっと起きたのかい"),
                ohiru: || String::from("もうお昼だよ"),
                kure: || String::from("もう日が暮れちまうよ"),
                yoru: || String::from("もう夜中だよ"),
            },
            konnichiwa: AisatsuHibiKonnichiwa {
                mayonaka: || String::from("こんな夜中にどうしたんだい?"),
                ohayo: || String::from("おや、早いんだねぇ"),
                konnichiwa: || String::from("はい、こんにちは"),
                konbanwa: || String::from("もう、こんばんは、だねぇ"),
            },
            konbanwa: AisatsuHibiKonbanwa {
                mayonaka: || String::from("こんな夜中にどうしたんだい?"),
                ohayo: || String::from("何言ってるんだい。もう朝だよ"),
                konnichiwa: || String::from("何言ってるんだい。まだお昼だよ"),
                konbanwa: || String::from("はい、こんばんは"),
            },
            oyasumi: AisatsuHibiOyasumi {
                mayonaka: || String::from("夜更かししないで早く寝るんだよ"),
                ohayo: || String::from("おや、今から寝るのかい。\nちゃんと寝るんだよ"),
                konnichiwa: || String::from("おや、風邪でもひいたのかい?\nゆっくり休むんだよ"),
                oyasumi: || String::from("はい、おやすみ"),
            },
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
            iroiro: |iroiro| format!("{}があるよ", iroiro.join("か\n")),
            aru: |mono| format!("はい、{}", mono),
            nai: |mono| format!("{}はないねぇ", mono),
            wakaran: |donna, mono| format!("{}{}はちょっとわからないねぇ", donna, mono),
            naikedo: |nai, nara, aru| {
                format!("{}はないけど\n{}なら\n{}があったかねぇ", nai, nara, aru)
            },
        },
        desuka: ToikakeDesuka {
            wakaran: |nai| format!("{}は扱ってないからよくわからないねぇ", nai),
            subete: || format!("そうだよ"),
            doreka: || format!("そうじゃないものもあるねぇ"),
            chigau: || format!("たぶん違うねぇ"),
            dayo: |are| format!("{}だねぇ", are),
            iroiro: |iroiro| format!("{}だねぇ", iroiro.join("か\n")),
            naniga: || format!("何がだい?"),
            ikura: |nefuda| {
                format!(
                    "{}だよ",
                    nefuda.iter().fold(String::new(), |s, (n, p)| format!(
                        "{}{}は￥{}\n",
                        s,
                        n,
                        p.to_string()
                    ))
                )
            },
        },
    },
    onegai: Onegai {
        shitai: || format!("周りに迷惑かけない範囲で好きにしておくれ"),
        shite: || format!("ばあちゃんは忙しいから自分でやっとくれ"),
        shiritai: OnegaiShiritai {
            kore: |nani| {
                format!(
                    "{}について知りたいのかい?\n開発中だからちょっと待っておくれ",
                    nani
                )
            },
            naniga: || {
                format!("何について知りたいのかい?\nと言っても教えてあげられる事は何もないねぇ")
            },
        },
    },
    tawainai: Tawainai {
        arigato: || String::from("はい、ありがとうね。"),
        douitashimashite: || String::from("お礼はいいから、もっと買っとくれ"),
        ayamaru: || String::from("おや、何かまずかったかね。すまないねぇ。"),
        bochibochi: || String::from("ばあちゃんはもう歳だからねぇ"),
        hagemasu: || String::from("大変だと思うけど、頑張っとくれ"),
        kizukai: || String::from("おや、大丈夫かい?\nしっかり休むんだよ"),
        nani: || {
            vec![
                "なんだい",
                "呼んだかい?",
                "どうかしたかい?",
                "何か言ったかい?",
            ]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
        },
        ocha: |m, yobareta| {
            if yobareta {
                format!("ばあちゃんは{}じゃないよ!\nきちんとした言葉をお使い!", m)
            } else {
                format!("{}がなんだって?\nきちんとした言葉をお使い!", m)
            }
        },
        sonota: || String::from("ホットサンド、ビールはいらんかねー?"),
        sounanda: || String::from("おや、そうなのかい。"),
        yokatta: || String::from("そうかい、それは良かったねぇ"),
    },
    wakaran: Wakaran {
        wakaran: || String::from("ちょっとわかんないねぇ"),
    },
    kitanai: |ng| format!("{}とはなんだい!\nもっと綺麗な言葉をお使い!", ng),
    error: Error {
        token: || String::from("おや、トークン取得でエラーかい"),
        parse: || String::from("おや、解析APIでエラーかい"),
        sentence: || String::from("おや、文タイプ判別APIでエラーかい"),
        noimpl: || String::from("おや、未実装な反応だね"),
    },
};
