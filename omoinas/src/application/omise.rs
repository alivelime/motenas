pub mod comfull_co_jp;
pub mod tokishirazu_llc;

use chrono::DateTime;

use crate::model::omise::*;

pub fn new(namae: &str) -> Omise {
    return match namae {
        "tokishirazu.llc/bachan" => Omise {
            id: String::from(namae),
            client: Client {
                id: "tokishirazu.llc",
                namae: "合同会社ときしらず",
                hp: "https://www.tokishirazu.llc",
            },
            namae: String::from("bachan"),
            hp: String::from("https://www.tokishirazu.llc"),
            yotei: String::from("24時間365日"),
            menu: tokishirazu_llc::shinkansen(),
            status: Status::Hima,
            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00")
                .unwrap()
                .format("%Y%m%d")
                .to_string(),
        },
        "comfull.co.jp/sendagi" => Omise {
            id: String::from(namae),
            client: Client {
                id: "comfull.co.jp",
                namae: "株式会社コンフル",
                hp: "https://comfull.co.jp",
            },
            namae: String::from("comfull sendagi"),
            hp: String::from("https://comfull.co.jp"),
            yotei: String::from("月〜金：11:00〜23:00\n土日祝：11:00〜23:00"),
            menu: comfull_co_jp::sendagi(),
            status: Status::Hima,
            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00")
                .unwrap()
                .format("%Y%m%d")
                .to_string(),
        },
        _ => panic!("missing omise.{}", namae),
    };
}

pub fn google_map_url(lat: f64, lng: f64) -> String {
    return format!(
        "https://www.google.com/maps/dir/?api=1&travelmode=transit&destination={},{}",
        lat, lng
    );
}
