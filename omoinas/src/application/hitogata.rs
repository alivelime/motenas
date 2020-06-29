pub mod comfull_co_jp;
pub mod tokishirazu_llc;

use crate::application::omise;
use crate::model::hitogata::Hitogata;
use crate::model::omise::OmiseRepo;

pub fn new<OR: OmiseRepo>(namae: &str) -> Hitogata {
    return match namae {
        "tokishirazu.llc/passengers/bachan" => Hitogata {
            id: String::from(namae),
            namae: "パッセンジャーズばあちゃん",
            kaeshi: &tokishirazu_llc::bachan::BACHAN,
            omise: omise::new::<OR>("tokishirazu.llc/passengers"),
        },
        "tokishirazu.llc/passengers/minarai" => Hitogata {
            id: String::from(namae),
            namae: "パッセンジャーズみならいちゃん",
            kaeshi: &tokishirazu_llc::minarai::MINARAI,
            omise: omise::new::<OR>("tokishirazu.llc/passengers"),
        },
        "comfull.co.jp/sendagi/bachan" => Hitogata {
            id: String::from(namae),
            namae: "コンフルばあちゃん",
            kaeshi: &comfull_co_jp::bachan::BACHAN,
            omise: omise::new::<OR>("comfull.co.jp/sendagi"),
        },
        "comfull.co.jp/sendagi/minarai" => Hitogata {
            id: String::from(namae),
            namae: "コンフルみならいちゃん",
            kaeshi: &comfull_co_jp::minarai::MINARAI,
            omise: omise::new::<OR>("comfull.co.jp/sendagi"),
        },
        _ => panic!("invalid chara. {}", namae),
    };
}
