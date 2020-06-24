use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::mono::Mono;

pub trait OmiseRepo {
    fn new() -> Self;
    fn get(&self, client_id: &String, omise_id: &String) -> Result<Omise, String>;
    fn put(&self, omise: &Omise) -> bool;
}

pub struct Client {
    pub id: &'static str,
    pub name: &'static str,
    pub hp: &'static str,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Omise {
    pub client_id: String,
    pub omise_id: String,
    pub namae: String,
    pub url: String,
    pub yotei: String,
    pub otokoro: Address,

    #[serde(skip)]
    pub oshinagaki: Vec<Mono>,
    pub omotenashi: HashSet<String>,

    pub ima: Status,
    pub hitokoto: String,
    pub aikotoba: String,
    pub kefu_kara: DateTime<FixedOffset>,
    pub kefu_made: DateTime<FixedOffset>,

    pub tanamono: HashSet<String>,

    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
impl Omise {
    pub fn new(client_id: String, omise_id: String) -> Omise {
        return Omise {
            client_id: client_id,
            omise_id: omise_id,
            namae: String::from(""),
            url: String::from(""),
            yotei: String::from(""),
            otokoro: Address::new(),
            oshinagaki: Vec::new(),
            omotenashi: HashSet::new(),

            ima: Status::Wakaran,
            hitokoto: String::from(""),
            aikotoba: String::from(""),
            kefu_kara: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),
            kefu_made: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),

            tanamono: HashSet::new(),

            created_at: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),
            updated_at: DateTime::parse_from_rfc3339("2020-01-01T00:00:00+09:00").unwrap(),
        };
    }
    pub fn menu(&self) -> Vec<&Mono> {
        return self.oshinagaki.iter().collect::<Vec<&Mono>>();
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Status {
    Wakaran,
    Yasumi,
    Hima,
    Bochibochi,
    Isogashi,
    Ippai,
    Kashikiri,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Address {
    pub country: String,
    pub postcode: u32,
    pub prefcode: u32,
    pub city: String,
    pub street: String,
    pub building: String,
    pub access: String,
}
impl Address {
    pub fn new() -> Address {
        return Address {
            country: String::from("jpn"),
            postcode: 1100001,
            prefcode: 13,
            city: String::from("千代田区"),
            street: String::from("千代田1-1"),
            building: String::from("デフォルト"),
            access: String::from("デフォルト"),
        };
    }
    pub fn for_map(&self) -> String {
        return format!("{}{}{}", self.todofuken(), self.city, self.street);
    }
    pub fn todofuken(&self) -> String {
        return String::from(match self.prefcode {
            01 => "北海道",
            02 => "青森県",
            03 => "岩手県",
            04 => "宮城県",
            05 => "秋田県",
            06 => "山形県",
            07 => "福島県",
            08 => "茨城県",
            09 => "栃木県",
            10 => "群馬県",
            11 => "埼玉県",
            12 => "千葉県",
            13 => "東京都",
            14 => "神奈川県",
            15 => "新潟県",
            16 => "富山県",
            17 => "石川県",
            18 => "福井県",
            19 => "山梨県",
            20 => "長野県",
            21 => "岐阜県",
            22 => "静岡県",
            23 => "愛知県",
            24 => "三重県",
            25 => "滋賀県",
            26 => "京都府",
            27 => "大阪府",
            28 => "兵庫県",
            29 => "奈良県",
            30 => "和歌山県",
            31 => "鳥取県",
            32 => "島根県",
            33 => "岡山県",
            34 => "広島県",
            35 => "山口県",
            36 => "徳島県",
            37 => "香川県",
            38 => "愛媛県",
            39 => "高知県",
            40 => "福岡県",
            41 => "佐賀県",
            42 => "長崎県",
            43 => "熊本県",
            44 => "大分県",
            45 => "宮崎県",
            46 => "鹿児島県",
            47 => "沖縄県",
            _ => "不明な都道府県",
        });
    }
}
