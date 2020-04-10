use crate::model::mono::{Mono, MonoResult, MonoType};

pub struct MonoRepository {}

pub fn get_menu() -> Vec<&'static str> {
    return vec!["お茶", "コーヒー"];
}

pub fn get_mono(nani: &String, donna: &Option<String>) -> MonoResult {
    let ice_coffee = Mono {
        name: String::from("アイスコーヒー"),
        donna: MonoType::Tsumetai,
    };
    let hot_coffee = Mono {
        name: String::from("ホットコーヒー"),
        donna: MonoType::Atatakai,
    };
    let atsui_ocha = Mono {
        name: String::from("熱いお茶"),
        donna: MonoType::Atatakai,
    };

    let mono_type = match &donna {
        Some(d) => match d.as_str() {
            "あたたかい" | "暖かい" | "温かい" | "あつい" | "熱い" => {
                Some(MonoType::Atatakai)
            }
            "冷たい" | "つめたい" => Some(MonoType::Tsumetai),
            _ => Some(MonoType::Other),
        },
        None => None,
    };

    return match nani.as_str() {
        "飲み物" | "ドリンク" => match &mono_type {
            Some(mt) => match mt {
                MonoType::Tsumetai => MonoResult::Mono(vec![ice_coffee]),
                MonoType::Atatakai => MonoResult::Mono(vec![hot_coffee, atsui_ocha]),
                _ => MonoResult::Wakaran(donna.as_ref().unwrap().clone(), nani.clone()),
            },
            None => MonoResult::Category(vec!["お茶", "コーヒー"]),
        },
        "お茶" => match &mono_type {
            Some(mt) => match mt {
                MonoType::Tsumetai => {
                    MonoResult::Kawari(format!("{:?}{}", donna, nani), vec![atsui_ocha])
                }
                MonoType::Atatakai => MonoResult::Mono(vec![atsui_ocha]),
                _ => MonoResult::Wakaran(donna.as_ref().unwrap().clone(), nani.clone()),
            },
            None => MonoResult::Mono(vec![atsui_ocha]),
        },
        "コーヒー" => match &mono_type {
            Some(mt) => match mt {
                MonoType::Tsumetai => MonoResult::Mono(vec![ice_coffee]),
                MonoType::Atatakai => MonoResult::Mono(vec![hot_coffee]),
                _ => MonoResult::Wakaran(donna.as_ref().unwrap().clone(), nani.clone()),
            },
            None => MonoResult::Mono(vec![hot_coffee, ice_coffee]),
        },
        _ => MonoResult::Nai(nani.clone()),
    };
}
