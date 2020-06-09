pub mod comfull_co_jp;
pub mod tokishirazu_llc;

use crate::application::omise;
use crate::model::hitogata::Hitogata;
use crate::model::omise::OmiseRepo;

pub fn new<OR: OmiseRepo>(namae: &str) -> Hitogata {
    return match namae {
        "tokishirazu.llc/bachan" => Hitogata {
            id: String::from(namae),
            namae: "パッセンジャーズばあちゃん",
            kaeshi: &tokishirazu_llc::bachan::BACHAN,
            omise: omise::new::<OR>(namae),
        },
        "tokishirazu.llc/minarai" => Hitogata {
            id: String::from(namae),
            namae: "みならいちゃん",
            kaeshi: &tokishirazu_llc::minarai::MINARAI,
            omise: omise::new::<OR>(namae),
        },
        "comfull.co.jp/sendagi/bachan" => Hitogata {
            id: String::from(namae),
            namae: "コンフルばあちゃん",
            kaeshi: &comfull_co_jp::bachan::BACHAN,
            omise: omise::new::<OR>("comfull.co.jp/sendagi"),
        },
        _ => panic!("invalid chara. {}", namae),
    };
}
