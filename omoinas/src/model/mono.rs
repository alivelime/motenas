enum MonoType {
    Tsumetai,
    Atatakai,

    Other,
}
/*
pub struct Mono {
    name: String,
    category: String,
    donna: Option<MonoType>,
}
*/

pub struct MonoRepository {}

pub fn get_mono(nani: &String, donna: &Option<String>) -> String {
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
                MonoType::Tsumetai => String::from("アイスコーヒーがあったかね"),
                MonoType::Atatakai => String::from("熱いお茶かホットコーヒーがあるよ"),
                _ => format!(
                    "はて、{}飲み物ねぇ?",
                    donna.as_ref().unwrap_or(&String::from(""))
                ),
            },
            None => String::from("お茶かコーヒーがあるよ"),
        },
        "お茶" => match &mono_type {
            Some(mt) => match mt {
                MonoType::Tsumetai => String::from("冷蔵庫で冷やしとくれ"),
                MonoType::Atatakai => String::from("淹れたてのお茶があるよ"),
                _ => format!(
                    "はて、{}お茶ねぇ?",
                    donna.as_ref().unwrap_or(&String::from(""))
                ),
            },
            None => String::from("淹れたてのお茶があるよ"),
        },
        "コーヒー" => match &mono_type {
            Some(mt) => match mt {
                MonoType::Tsumetai => String::from("アイスコーヒーがあるよ"),
                MonoType::Atatakai => String::from("ホットコーヒーがあるよ"),
                _ => format!(
                    "はて、{}コーヒーねぇ?",
                    donna.as_ref().unwrap_or(&String::from(""))
                ),
            },
            None => String::from("ホットかアイスコーヒーがあるよ"),
        },
        _ => format!("{}はないねぇ", nani),
    };
}
