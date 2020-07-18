use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use std::env;

use crate::model::omise::*;

#[derive(Deserialize, Debug)]
pub struct Event {
    client_id: Option<String>,
    omise_id: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub omise: Vec<Omise>,
}
pub fn main<OR: OmiseRepo>(_: Event) -> Result<Response, String> {
    let or = OR::new();
    let omise = vec![
        Omise {
            client_id: String::from("tokishirazu.llc"),
            omise_id: String::from("passengers"),
            namae: String::from("パッセンジャーズ(仮)"),
            link: Links {
                hp: String::from("https://www.tokishirazu.llc"),
                twitter: String::from("https://twitter.com/purejapaneseonl"),
                facebook: String::from("https://www.facebook.com/profile.php?id=100009973449056"),
                instagram: String::from(""),
                line: if env::var("ENV").unwrap() == "dev" {
                    String::from("https://lin.ee/moLu1Kr")
                } else {
                    String::from("https://lin.ee/E5krmZP")
                },
            },
            yotei: String::from("24時間365日"),
            oshirase: String::from("コロナの影響により、一部商品が画像表示されません。"),
            otokoro: Address {
                country: String::from("jpn"),
                postcode: 1530063 as u32,
                prefcode: 13 as u32,
                city: String::from("目黒区"),
                street: String::from("目黒2-11-3"),
                building: String::from("印刷工場1F"),
                access: String::from("目黒駅から徒歩10分"),
            },
            oshinagaki: Vec::new(),
            omotenashi: vec![
                String::from("cafe"),
                String::from("wifi"),
                String::from("alcohol"),
                String::from("non-smoking"),
                String::from("restroom"),
                String::from("飲み物"),
                String::from("お土産"),
                String::from("弁当"),
                String::from("軽食"),
                String::from("お菓子"),
                String::from("雑誌"),
            ]
            .into_iter()
            .collect(),
            oshiharai: vec![String::from("cash"), String::from("交通系IC")]
                .into_iter()
                .collect(),
            tanamono: vec![
                String::from("Ube2da389e4c223405286c03a32fefcb6"),
                String::from("U15022dc52eb46f56a4ea7d7ee3fcaebe"),
                String::from("U82c5d6f8e6c60a393e79ed7294c09202"),
                String::from("U51dbd53df87a0c68fb90bad2af248fc7"),
            ]
            .into_iter()
            .collect(),

            ima: vec![
                Ima {
                    namae: String::from("グリーン席"),
                    status: Status::Bochibochi,
                },
                Ima {
                    namae: String::from("普通席"),
                    status: Status::Manseki,
                },
            ],
            hitokoto: String::from(""),
            kefu_kara: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
            kefu_made: DateTime::parse_from_rfc3339("2020-06-02T00:00:00+09:00").unwrap(),

            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
            updated_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
        },
        Omise {
            client_id: String::from("comfull.co.jp"),
            omise_id: String::from("sendagi"),
            namae: String::from("コンフル千駄木店"),
            link: Links {
                hp: String::from("https://comfull.co.jp"),
                twitter: String::from("https://twitter.com/cowork_comfull"),
                facebook: String::from(""),
                instagram: String::from(""),
                line: if env::var("ENV").unwrap() == "dev" {
                    String::from("https://lin.ee/msYlf1q")
                } else {
                    String::from("https://lin.ee/Rli2RvT")
                },
            },
            yotei: String::from("月〜金：11:00〜22:00\n土日祝：11:00〜22:00"),
            oshirase: String::from("コロナに伴う営業時間変更のお知らせ"),
            otokoro: Address {
                country: String::from("jpn"),
                postcode: 1130022 as u32,
                prefcode: 13 as u32,
                city: String::from("文京区"),
                street: String::from("千駄木2-33-8"),
                building: String::from("TKB千駄木ビル2F-3F"),
                access: String::from("千代田線千駄木駅から徒歩1分"),
            },
            oshinagaki: Vec::new(),
            omotenashi: vec![
                String::from("cafe"),
                String::from("wifi"),
                String::from("smoking"),
                String::from("alcohol"),
                String::from("non-smoking"),
                String::from("restroom"),
                String::from("plug"),
                String::from("軽食"),
                String::from("フリードリンク"),
                String::from("各種レンタル"),
            ]
            .into_iter()
            .collect(),
            oshiharai: vec![
                String::from("cash"),
                String::from("visa"),
                String::from("master"),
                String::from("jcb"),
                String::from("amex"),
                String::from("diners"),
                String::from("discover"),
                String::from("交通系IC"),
                String::from("applepay"),
                String::from("paypay"),
                String::from("alipay"),
                String::from("linepay"),
                String::from("iD"),
                String::from("quickpay"),
                String::from("aupay"),
                String::from("docomo"),
            ]
            .into_iter()
            .collect(),
            tanamono: vec![
                String::from("Ube2da389e4c223405286c03a32fefcb6"),
                String::from("U60b51e87ae2bbf7f08340a6b429ccbf9"),
                String::from("Ua3c455486f0f1f6f73fcce07a161d064"),
            ]
            .into_iter()
            .collect(),

            ima: vec![
                Ima {
                    namae: String::from("2階(禁煙)"),
                    status: Status::Hima,
                },
                Ima {
                    namae: String::from("3階(喫煙)"),
                    status: Status::Hima,
                },
            ],
            hitokoto: String::from("今日も営業中です"),
            kefu_kara: DateTime::parse_from_rfc3339("2020-06-01T10:00:00+09:00").unwrap(),
            kefu_made: DateTime::parse_from_rfc3339("2020-06-01T22:00:00+09:00").unwrap(),

            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
            updated_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
        },
    ];

    for o in &omise {
        or.put(o);
    }

    return Ok(Response { omise: omise });
}
