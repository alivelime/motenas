use rand::seq::SliceRandom;

use crate::model::hitogata::*;

pub const NORIMARU: Kaeshi = Kaeshi {
    aisatsu: Aisatsu {
        hibi: AisatsuHibi {
            ohayo: AisatsuHibiOhayo {
                mayonaka: || String::from("おはようございます!"),
                akegata: || String::from("おはようございます!"),
                ohayo: || String::from("おはようございます!"),
                osoyo: || String::from("おはようございます!"),
                ohiru: || String::from("おはようございます!"),
                kure: || String::from("おはようございます!"),
                yoru: || String::from("おはようございます!"),
            },
            konnichiwa: AisatsuHibiKonnichiwa {
                mayonaka: || String::from("こんにちは！"),
                ohayo: || String::from("こんにちは！"),
                konnichiwa: || String::from("こんにちは！"),
                konbanwa: || String::from("こんにちは！"),
            },
            konbanwa: AisatsuHibiKonbanwa {
                mayonaka: || String::from("こんばんは！"),
                ohayo: || String::from("こんばんは！"),
                konnichiwa: || String::from("こんばんは！"),
                konbanwa: || String::from("こんばんは！"),
            },
            oyasumi: AisatsuHibiOyasumi {
                mayonaka: || String::from("おやすみなさい。お疲れ様でした！"),
                ohayo: || String::from("おやすみなさい。お疲れ様でした！<Paste>"),
                konnichiwa: || String::from("おやすみなさい。お疲れ様でした！<Paste>"),
                oyasumi: || String::from("おやすみなさい。お疲れ様でした！"),
            },
        },
        iku: AisatsuIku {
            matane: AisatsuIkuMatane {
                toki: |itsu| format!("また{}お会いしましょう", itsu),
                tokoro: |doko| format!("また{}でお会いしましょう", doko),
                mata: || String::from("ありがとうございます！\nまた、いつでも来てください！"),
            },
            sayonara: || String::from("はい、さようなら！"),
            ittera: AisatsuIkuIttera {
                toki: |_| String::from("行ってらっしゃい！"),
                tokoro: |_| String::from("行ってらっしゃい！"),
                tokitokoro: |_, _| String::from("行ってらっしゃい！"),
                ittera: || String::from("はい、行ってらっしゃい\nマスクは持ったかい?"),
            },
        },
        kuru: AisatsuKuru {
            hajimemashite: || String::from("はじめまして！\ncomfullののりまるです！"),
            hisashiburi: || String::from("お久しぶりです！お元気してますか？"),
            okaeri: || String::from("お帰りなさいませ！"),
            irasshai: || String::from("いらっしゃいませ！\nいつもありがとうございます！"),
        },
    },
    shirase: Shirase {
        iku: ShiraseIku {},
        kuru: ShiraseKuru {},
    },
    toikake: Toikake {
        aru: ToikakeAru {
            iroiro: |iroiro| format!("{}があります!", iroiro.join("か\n")),
            aru: |mono| format!("はい、{}はこちらです！", mono),
            nai: |mono| format!("申し訳ありません。{}はご用意しておりません。", mono),
            wakaran: |donna, mono| {
                format!(
                    "申し訳ありません。{}{}はちょっとわからないので、直接店長にお聞きください。",
                    donna, mono
                )
            },
            naikedo: |nai, nara, aru| {
                format!(
                    "申し訳ありません。{}はないです。\n{}なら\n{}でしたらございます",
                    nai, nara, aru
                )
            },
        },
        desuka: ToikakeDesuka {
            wakaran: |nai| format!("申し訳ありません。{}のお取り扱いはございません。", nai),
            subete: || format!("はい、その通りだと思います！"),
            doreka: || format!("そうでないものもございますね！"),
            chigau: || format!("いいえ、違うと思います！"),
            dayo: |are| format!("{}だと思います！", are),
            iroiro: |iroiro| format!("{}だと思います", iroiro.join("か\n")),
            naniga: || format!("何についてお聞きですか？"),
            ikura: |nefuda| {
                format!(
                    "{}なります！",
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
        shitai: || format!("ご注文や備品のご利用希望でしたら、メニューのご注文からどうぞ。"),
        shite: || {
            format!("ご注文や備品のご利用希望でしたら、メニューのご注文からどうぞ。その他のご要望は直接スタッフまでお声がけください！")
        },
        shiritai: OnegaiShiritai {
            kore: |nani| {
                format!(
                    "申し訳ありません。{}はちょっとわからないので、直接店長にお聞きください。",
                    nani
                )
            },
            naniga: || format!("何についてお知りになりたいのでしょうか?"),
        },
    },
    tawainai: Tawainai {
        arigato: || String::from("はい、ありがとうございます！"),
        douitashimashite: || String::from("お役に立てて嬉しいです！"),
        ayamaru: || String::from("申し訳ございません、何か間違えましたでしょうか？"),
        bochibochi: || String::from("はい、とても元気です！"),
        hagemasu: || String::from("はい！ただ、無理はしないでください！"),
        kizukai: || String::from("大丈夫ですか？\n風邪薬を飲むか、病院へ向かいましょう！"),
        nani: || {
            vec![
                "はい、何でしょうか?",
                "呼びましたでしょうか?",
                "どうかされましたか？",
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
        sonota: || String::from("コンフルにようこそ。ご注文はメニューの注文画面より承ります！"),
        sounanda: || String::from("そうなんですね！"),
        yokatta: || String::from("それは何よりです！"),
    },
    wakaran: Wakaran {
        wakaran: || String::from("申し訳ありません。わかりません！"),
    },
    kitanai: |_| String::from("申し訳ありません。NGワードが含まれています"),
    error: Error {
        token: || String::from("申し訳ありません。エラーが発生しました！"),
        parse: || String::from("申し訳ありません。エラーが発生しました！"),
        sentence: || String::from("申し訳ありません。エラーが発生しました！"),
        noimpl: || String::from("申し訳ありません。エラーが発生しました！"),
    },
};
