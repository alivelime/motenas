pub mod comfull_co_jp;
pub mod tokishirazu_llc;

use chrono::{DateTime, FixedOffset, Utc};

use crate::model::omise::*;

pub fn new(namae: &str) -> Omise {
    return match namae {
        "tokishirazu.llc/bachan" => Omise {
            client_id: String::from("tokishirazu.llc"),
            omise_id: String::from("bachan"),
            namae: String::from("bachan"),
            url: String::from("https://www.tokishirazu.llc"),
            yotei: String::from("24時間365日"),
            otokoro: Address {
                country: String::from("jpn"),
                postcode: 1530063 as u32,
                prefcode: 13 as u32,
                city: String::from("目黒区"),
                street: String::from("目黒2-11-3"),
                building: String::from("印刷工場1F"),
                access: String::from("目黒駅から徒歩10分"),
            },
            oshinagaki: tokishirazu_llc::shinkansen(),

            ima: Status::Wakaran,
            kefu_kara: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
            kefu_made: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),

            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
            updated_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
        },
        "comfull.co.jp/sendagi" => Omise {
            client_id: String::from("comfull.co.jp"),
            omise_id: String::from("sendagi"),
            namae: String::from("コンフル千駄木店"),
            url: String::from("https://comfull.co.jp"),
            yotei: String::from("月〜金：11:00〜23:00\n土日祝：11:00〜23:00"),
            otokoro: Address {
                country: String::from("jpn"),
                postcode: 1530063 as u32,
                prefcode: 13 as u32,
                city: String::from("目黒区"),
                street: String::from("目黒2-11-3"),
                building: String::from("印刷工場1F"),
                access: String::from("目黒駅から徒歩10分"),
            },
            oshinagaki: comfull_co_jp::sendagi(),

            ima: Status::Wakaran,
            kefu_kara: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
            kefu_made: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),

            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
            updated_at: Utc::now().with_timezone(&FixedOffset::east(9 * 3600)),
        },
        _ => panic!("missing omise.{}", namae),
    };
}

pub fn google_map_url(address: String) -> String {
    return format!(
        "https://www.google.com/maps/dir/?api=1&travelmode=transit&destination={}",
        address,
    );
}
