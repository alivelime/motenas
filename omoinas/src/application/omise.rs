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
            namae: "bachan",
            hp: "https://www.tokishirazu.llc",
            yotei: "24時間365日",
            menu: tokishirazu_llc::shinkansen(),
            status: Status::Hima,
            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
        },
        "comfull.co.jp/sendagi" => Omise {
            id: String::from(namae),
            client: Client {
                id: "comfull.co.jp",
                namae: "株式会社コンフル",
                hp: "https://comfull.co.jp",
            },
            namae: "comfull sendagi",
            hp: "https://comfull.co.jp",
            yotei: "月〜金：11:00〜23:00\n土日祝：11:00〜23:00",
            menu: comfull_co_jp::sendagi(),
            status: Status::Hima,
            created_at: DateTime::parse_from_rfc3339("2020-06-01T00:00:00+09:00").unwrap(),
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
