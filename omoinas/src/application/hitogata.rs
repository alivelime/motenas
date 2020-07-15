pub mod comfull_co_jp;
pub mod tokishirazu_llc;

use log::error;

use crate::application::omise;
use crate::model::hitogata::Hitogata;
use crate::model::omise::OmiseRepo;

pub fn new<OR: OmiseRepo>(namae: &str) -> Hitogata {
    // 一旦、1店舗1キャラにしとく
    return match namae {
        "tokishirazu.llc/passengers" => Hitogata {
            id: String::from(namae),
            namae: String::from("ばあちゃん"),
            icon: String::from(
                "https://d261s45nifjpgc.cloudfront.net/images/tokishirazu_llc/bachan.png",
            ),
            kaeshi: &tokishirazu_llc::bachan::BACHAN,
            omise: omise::new::<OR>("tokishirazu.llc/passengers"),
        },
        "tokishirazu.llc/passengers/minarai" => Hitogata {
            id: String::from(namae),
            namae: String::from("みならいちゃん"),
            icon: String::from(
                "https://d261s45nifjpgc.cloudfront.net/images/tokishirazu_llc/minarai.png",
            ),
            kaeshi: &tokishirazu_llc::minarai::MINARAI,
            omise: omise::new::<OR>("tokishirazu.llc/passengers"),
        },
        "comfull.co.jp/sendagi" => Hitogata {
            id: String::from(namae),
            namae: String::from("のりまるくん"),
            icon: String::from(
                "https://d261s45nifjpgc.cloudfront.net/images/comfull_co_jp/norimaru.png",
            ),
            kaeshi: &comfull_co_jp::norimaru::NORIMARU,
            omise: omise::new::<OR>("comfull.co.jp/sendagi"),
        },
        _ => {
            error!("invalid chara. {}", namae);
            panic!("invalid chara. {}", namae);
        }
    };
}
