use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Aru {
    omomuki: omomuki::Omomuki,
}

impl Aru {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.dare.is_none()
            && if let Some(d) = &omomuki.doushita {
                d.suru == "ある" || d.suru == "ない" || d.suru == "貰える" || d.suru == "くれる"
            } else {
                false
            }
        {
            return Some(Box::new(Aru {
                omomuki: omomuki.clone(),
            }));
        }
        return None;
    }
}
impl Tumori for Aru {
    fn get_kotae(&self) -> String {
        if let Some(nani) = &self.omomuki.nani {
            if nani == "飲み物" || nani == "ドリンク" {
                if let Some(donoyouni) = &self.omomuki.donoyouni {
                    match donoyouni.as_str() {
                        "あたたかい" | "暖かい" | "温かい" | "あつい" | "熱い" => {
                            return String::from("あったかいお茶かコーヒーがあるよ");
                        }
                        "冷たい" | "つめたい" => {
                            return String::from("アイスコーヒーがあったかねぇ");
                        }
                        "美味しい" | "おいしい" => {
                            return String::from("熱いお茶が一番だねぇ");
                        }
                        _ => {
                            return format!("はて、{}飲み物ねぇ", donoyouni.as_str());
                        }
                    };
                }
                return String::from("お茶かコーヒーでいいかい?");
            }
            if nani == "お茶" {
                if let Some(donoyouni) = &self.omomuki.donoyouni {
                    match donoyouni.as_str() {
                        "あたたかい" | "暖かい" | "温かい" | "あつい" | "熱い" => {
                            return String::from("はい、お茶🍵\n熱いから気をつけるんだよ");
                        }
                        "冷たい" => {
                            return String::from("冷蔵庫にあったかねぇ");
                        }
                        "美味しい" | "おいしい" => {
                            return String::from("はい、おいしいお茶が入ったよ🍵");
                        }
                        _ => {
                            return format!("はて、{}お茶は飲んだことないねぇ", donoyouni.as_str());
                        }
                    };
                }

                return String::from("はい、お茶🍵");
            }
            if nani == "コーヒー" {
                if let Some(donoyouni) = &self.omomuki.donoyouni {
                    match donoyouni.as_str() {
                        "あたたかい" | "暖かい" | "温かい" | "あつい" | "熱い" => {
                            return String::from("はい、コーヒー☕️\n熱いから気をつけるんだよ");
                        }
                        "冷たい" => {
                            return String::from("冷たいのはばあちゃん飲まないねぇ");
                        }
                        "美味しい" | "おいしい" => {
                            return String::from("はい、淹れたてのコーヒー");
                        }
                        _ => {
                            return format!(
                                "はて、{}コーヒーは飲んだことないねぇ",
                                donoyouni.as_str()
                            );
                        }
                    };
                }
                return String::from("はい、コーヒー☕️。\n砂糖とミルクは自分で取っておくれ");
            }
            return format!("{}はないねぇ", nani);
        } else {
            return String::from("何が欲しいんだい?");
        }
    }
}
