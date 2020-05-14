use rand::seq::SliceRandom;

use crate::hitogata::*;

pub const MINARAI: Kaeshi = Kaeshi {
    aisatsu: Aisatsu {
        hibi: AisatsuHibi {
            ohayo: AisatsuHibiOhayo {
                mayonaka: || String::from("zzz"),
                akegata: || String::from("おはよーございます。 zzz"),
                ohayo: || String::from("おはようございます!"),
                osoyo: || String::from("おそようございます!"),
                ohiru: || String::from("お昼ですよ?\nあ!ひょっとしてインド辺りにお住まいですか?"),
                kure: || String::from("もう夕方ですよ!?\nひょっとして英国紳士・淑女ですか?"),
                yoru: || String::from("もう夜中ですよ?\nひょっとしてアメリカ西海岸在住の方ですか?"),
            },
            konnichiwa: AisatsuHibiKonnichiwa {
                mayonaka: || String::from("本日の営業は終了しました!"),
                ohayo: || String::from("おはようございます!"),
                konnichiwa: || String::from("はい、こんにちは"),
                konbanwa: || String::from("もう、こんばんはですよ?"),
            },
            konbanwa: AisatsuHibiKonbanwa {
                mayonaka: || String::from("本日の営業は終了しました"),
                ohayo: || String::from("こちらは朝です!"),
                konnichiwa: || String::from("太平洋諸島にお住まいですか?"),
                konbanwa: || String::from("はい、こんばんは"),
            },
            oyasumi: AisatsuHibiOyasumi {
                mayonaka: || String::from("早く寝てください"),
                ohayo: || String::from("夜勤ですか?\nお疲れ様です!"),
                konnichiwa: || String::from("風邪ですか?\nちゃんと寝てくださいね"),
                oyasumi: || String::from("はい、おやすみ"),
            },
        },
        iku: AisatsuIku {
            matane: AisatsuIkuMatane {
                toki: |itsu| format!("はい、また{}", itsu),
                tokoro: |doko| format!("はい、また{}で", doko),
                mata: || String::from("また、いつでも来てください"),
            },
            sayonara: || String::from("はい、さようなら"),
            ittera: AisatsuIkuIttera {
                toki: |itsu| format!("{}に行くんですか?\n行ってらっしゃい!", itsu),
                tokoro: |doko| format!("{}に行くんですか?\n行ってらっしゃい!", doko),
                tokitokoro: |itsu, doko| {
                    format!("{}{}に行くんですか?\n行ってらっしゃい!", itsu, doko)
                },
                ittera: || String::from("はい、行ってらっしゃい\nマスクは持ちました?"),
            },
        },
        kuru: AisatsuKuru {
            hajimemashite: || String::from("はい、はじめまして"),
            hisashiburi: || String::from("お久しぶりです!"),
            okaeri: || String::from("お帰りなさい"),
            irasshai: || String::from("いらっしゃいませ。何か買ってください"),
        },
    },
    shirase: Shirase {
        iku: ShiraseIku {},
        kuru: ShiraseKuru {},
    },
    toikake: Toikake {
        aru: ToikakeAru {
            iroiro: |iroiro| format!("{}があります。どれにしますか?", iroiro.join("か\n")),
            aru: |mono| format!("どうぞ、{}です", mono),
            nai: |mono| format!("すみません。{}はないです", mono),
            wakaran: |donna, mono| format!("すみません。{}{}はちょっとわからないです", donna, mono),
            naikedo: |nai, nara, aru| {
                format!("{}はないですけど\n{}なら\n{}がありますよ", nai, nara, aru)
            },
        },
        desuka: ToikakeDesuka {
            wakaran: |nai| format!("{}は扱ってないからちょっとわからないです", nai),
            subete: || format!("そうだと思います"),
            doreka: || format!("そうでないものもありますね"),
            chigau: || format!("違うと思います"),
            dayo: |are| format!("{}ですね", are),
            iroiro: |iroiro| format!("{}ですね", iroiro.join("か\n")),
            naniga: || format!("何がですか?"),
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
        shitai: || format!("迷惑かけない範囲でご自由にどうぞ"),
        shite: || format!("いやです"),
        shiritai: OnegaiShiritai {
            kore: |nani| format!("{}についてですか?\n開発中なので開発費ください", nani),
            naniga: || format!("何について知りたいんですか?\nと言っても未実装です"),
        },
    },
    tawainai: Tawainai {
        arigato: || String::from("はい、ありがとうございます!"),
        douitashimashite: || {
            String::from("お役に立てて嬉しいです。でももっと買ってくれるとさらに嬉しいです")
        },
        ayamaru: || String::from("すみません。とりあえず謝っておきます"),
        bochibochi: || String::from("誰か買ってください"),
        hagemasu: || String::from("そんなに頑張らなくていいんですよ?"),
        kizukai: || String::from("大丈夫ですか?\n今すぐ薬局にどうぞ"),
        nani: || {
            vec![
                "はい、何でしょう?",
                "呼びました?",
                "どうかしましたか?",
                "何か言いましたか?",
            ]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
        },
        ocha: |m, yobareta| {
            if yobareta {
                format!("{}ですか?", m)
            } else {
                format!("{}ですか?", m)
            }
        },
        sonota: || {
            String::from(
                "お弁当、ホットコーヒー、サンドイッチ、ビールにおつまみはいかがでしょうか?",
            )
        },
        sounanda: || String::from("へー、そうなんですね"),
        yokatta: || String::from("そうですか。それは良かったです"),
    },
    wakaran: Wakaran {
        wakaran: || String::from("ちょっと何言ってるかわからないです。\nなにしろbotなもので"),
    },
    kitanai: |ng| format!("{}はNGワードです\nブロックしますよ?", ng),
    error: Error {
        parse: || String::from("解析APIでエラーっぽいです"),
        sentence: || String::from("文タイプ判別APIでエラーみたいです"),
        noimpl: || String::from("未実装な反応です"),
    },
};
