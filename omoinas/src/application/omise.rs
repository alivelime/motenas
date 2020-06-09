pub mod comfull_co_jp;
pub mod tokishirazu_llc;

use crate::model::omise::*;

pub fn new<OR: OmiseRepo>(namae: &str) -> Omise {
    let or = OR::new();
    let ids: Vec<&str> = namae.split("/").collect();
    match or.get(&ids[0].to_string(), &ids[1].to_string()) {
        Ok(mut omise) => {
            match namae {
                "tokishirazu.llc/bachan" => {
                    omise.oshinagaki = tokishirazu_llc::shinkansen();
                    return omise;
                }
                "comfull.co.jp/sendagi" => {
                    omise.oshinagaki = comfull_co_jp::sendagi();
                    return omise;
                }
                _ => panic!("missing omise.{}", namae),
            };
        }
        Err(_) => {
            panic!("missing omise.{}", namae);
        }
    };
}

pub fn google_map_url(address: String) -> String {
    return format!(
        "https://www.google.com/maps/dir/?api=1&travelmode=transit&destination={}",
        address,
    );
}
