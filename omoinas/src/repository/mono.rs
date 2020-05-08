use std::collections::HashSet;

use crate::model::mono::{Desu, Mono, MonoResult};
use crate::model::{Koto, Kotoba, Nani};

pub struct MonoRepository<'a> {
    pub monos: Vec<&'a Mono>,
}

impl<'a> MonoRepository<'a> {
    pub fn new(m: Vec<&Mono>) -> MonoRepository {
        return MonoRepository { monos: m };
    }

    pub fn len(&self) -> usize {
        return self.monos.len();
    }
    pub fn get_menu(&self) -> Vec<String> {
        match self.len() {
            0 => {
                return Vec::new();
            }
            1 => {
                return vec![self.monos[0].category.last().unwrap().to_string()];
            }
            _ => {
                let mut menu = HashSet::new();
                for i in 0..self.monos[0].category.len() {
                    menu = HashSet::new();
                    for mono in &self.monos {
                        if i < mono.category.len() {
                            menu.insert(mono.category[i].as_str());
                        }
                    }

                    if menu.len() >= 2 {
                        return menu.iter().map(|m| m.to_string()).collect();
                    }
                }

                return menu.iter().map(|m| m.to_string()).collect();
            }
        };
    }

    pub fn filter_fuda(&self, nani: &Koto) -> MonoRepository {
        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| m.fuda.iter().any(|f| f == nani))
                .cloned()
                .collect(),
        };
    }

    pub fn match_all(&self, nani: Vec<Koto>) -> MonoRepository {
        let namae = nani
            .iter()
            .rev()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("");
        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| {
                    nani.iter()
                        .all(|n| m.category.iter().any(|c| c == n) || m.fuda.iter().any(|f| f == n))
                        || m.category.iter().any(|c| c == &namae)
                        || m.fuda.iter().any(|f| f == &namae)
                })
                .cloned()
                .collect(),
        };
    }
    pub fn korekana(&self, nani: &Vec<Koto>) -> MonoRepository {
        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| {
                    m.category.iter().any(|c| c == &nani[0]) || m.fuda.iter().any(|f| f == &nani[0])
                })
                .cloned()
                .collect(),
        };
    }
    pub fn get_result(&self, nai: Option<(String, String)>) -> MonoResult {
        return if let Some((nai, nara)) = nai {
            // 絞ったものがなければ、代替案を提示する
            MonoResult::Naikedo(nai.clone(), nara.clone(), self.get_menu())
        } else {
            match self.len() {
                1 | 2 | 3 => MonoResult::Mono(self.monos.iter().map(|m| (*m).clone()).collect()),
                _ => MonoResult::Category(self.get_menu()),
            }
        };
    }
    pub fn get_category(&self, nani: &String) -> Option<String> {
        return self.monos.iter().find_map(|&m| {
            if let Some(p) = m.category.iter().position(|c| c == nani) {
                Some(m.category[p - 1].to_string())
            } else {
                None
            }
        });
    }
    pub fn include_all(&self, kore: &Vec<Koto>) -> bool {
        return self.monos.iter().all(|m| {
            kore.iter()
                .all(|k| m.category.iter().any(|c| c == k) || m.fuda.iter().any(|f| f == k))
        });
    }
    pub fn include_any(&self, kore: &Vec<Koto>) -> bool {
        return self.monos.iter().any(|m| {
            kore.iter()
                .all(|k| m.category.iter().any(|c| c == k) || m.fuda.iter().any(|f| f == k))
        });
    }
}

pub fn is_mono(nani: &Nani) -> bool {
    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());
    let korekana = monos.korekana(&nani.mono);
    if korekana.len() > 0 {
        return true;
    }
    return false;
}
pub fn get_mono(nani: Option<&Nani>) -> MonoResult {
    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());
    if let Some(nani) = nani {
        // 冷たいチョコアイス?
        if let Some(donna) = &nani.donna {
            let searched = monos.match_all([nani.mono.clone(), vec![donna.clone()]].concat());
            if searched.len() > 0 {
                return searched.get_result(None);
            }
        }

        // チョコアイス?
        let searched = monos.match_all(nani.mono.clone());
        if searched.len() > 0 {
            return if nani.donna.is_some() {
                searched.get_result(Some((nani.donna_namae(), nani.namae())))
            } else {
                searched.get_result(None)
            };
        }

        // アイス?
        if nani.mono.len() >= 2 {
            let korekana = monos.korekana(&nani.mono);
            if korekana.len() > 0 {
                return korekana.get_result(Some((nani.namae(), nani.mono[0].to_string())));
            }
        }

        return MonoResult::Nai(nani.clone());
    } else {
        return MonoResult::Category(monos.get_menu());
    }
}

pub fn zakkuri(kore: &Nani) -> Desu {
    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());
    if let Some(category) = monos.get_category(&kore.namae()) {
        return Desu::IsCategory(category);
    }
    return match get_mono(Some(&kore)) {
        MonoResult::Category(category) => Desu::Category(category),
        MonoResult::Mono(mono) => Desu::Mono(mono),
        MonoResult::Naikedo(nai, _, _) => Desu::Wakaran(nai),
        MonoResult::Nai(mono) => Desu::Wakaran(mono.donna_namae()),
    };
}
pub fn ikura(kore: &Nani) -> Desu {
    return match get_mono(Some(&kore)) {
        MonoResult::Category(category) => Desu::Category(category),
        MonoResult::Mono(mono) => Desu::Ikura(mono),
        MonoResult::Naikedo(nai, _, _) => Desu::Wakaran(nai),
        MonoResult::Nai(mono) => Desu::Wakaran(mono.donna_namae()),
    };
}

pub fn desuka(kore: &Nani, are: &Nani) -> Desu {
    if kore.donna_namae() == are.donna_namae() {
        return Desu::Subete();
    }

    if are.mono.len() > 0 && are.has(vec!["何"]) {
        return zakkuri(kore);
    }
    if are.mono.len() > 0 && are.has(vec!["幾等", "おいくら"]) {
        return ikura(kore);
    }

    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());

    // 冷たいチョコアイス?
    let searched = monos.match_all(match &kore.donna {
        Some(donna) => kore.mono.iter().chain(vec![donna]).cloned().collect(),
        None => kore.mono.clone(),
    });

    return match searched.len() {
        0 => Desu::Wakaran(kore.donna_namae()),
        _ => {
            let mut nani = are.mono.clone();
            if let Some(donna) = &are.donna {
                nani.push(donna.clone())
            };

            if searched.include_all(&nani) {
                Desu::Subete {}
            } else if searched.include_any(&nani) {
                Desu::Doreka {}
            } else {
                Desu::Chigau {}
            }
        }
    };
}

fn get_data() -> Vec<Mono> {
    return vec![
        Mono {
            namae: "車内販売メニュー",
            category: vec![Kotoba::from_str("モノ"), Kotoba::from_str("メニュー")],
            fuda: vec![],
            neuchi: 0,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/topics/get_image?id=215&image_number=1",
            url: "https://www.jr-cp.co.jp/pdf/service/wagon_menu_list.pdf",
        },
        Mono {
            namae: "スジャータアイスクリーム(バニラ)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["アイス", "アイスクリーム"]),
                Kotoba::from_vec(vec!["バニラアイス", "バニラアイスクリーム"]),
            ],
            fuda: vec!["冷たい", "甘い", "バニラ", "スジャータ"],
            neuchi: 290,
            allergen: Some(vec!["乳", "卵"]),
            calorie: Some(232),
            image: "https://www.jr-cp.co.jp/services/38/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "スジャータアイスクリーム(いちご)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["アイス", "アイスクリーム"]),
                Kotoba::from_vec(vec!["ストロベリーアイス", "ストロベリーアイスクリーム"]),
            ],
            fuda: vec![
                "冷たい",
                "甘い",
                "苺",
                "ストロベリー",
                "スジャータ",
                "期間限定",
            ],
            neuchi: 330,
            allergen: Some(vec!["乳"]),
            calorie: Some(162),
            image: "https://www.jr-cp.co.jp/services/281/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "スジャータアイスクリーム(ピスタチオ)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["アイス", "アイスクリーム"]),
                Kotoba::from_str("ピスタチオアイス"),
            ],
            fuda: vec![
                "冷たい",
                "甘い",
                "ピスタチオ",
                "スジャータ",
                "新しい",
                "期間限定",
            ],
            neuchi: 390,
            allergen: Some(vec!["乳"]),
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/276/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "ウェッジ月刊誌",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("読み物"),
                Kotoba::from_str("雑誌"),
                Kotoba::from_vec(vec!["ウェッジ", "wedge"]),
            ],
            fuda: vec![],
            neuchi: 550,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/50/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "ひととき",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("読み物"),
                Kotoba::from_str("雑誌"),
                Kotoba::from_str("ひととき"),
            ],
            fuda: vec![],
            neuchi: 550,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/51/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "オーボンヴュータン／プティフールセック",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["新幹線スイーツ", "スイーツ"]),
                Kotoba::from_str("プティフールセック"),
            ],
            fuda: vec!["甘い", "焼き菓子", "オーボンヴュータン"],
            neuchi: 590,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/41/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "メゾンカイザー　フランボワーズショコラフィナンシェ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["新幹線スイーツ", "スイーツ"]),
                Kotoba::from_str("ショコラフィナンシェ"),
            ],
            fuda: vec!["甘い", "焼き菓子", "メゾンカイザー"],
            neuchi: 540,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/255/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "メゾンカイザー　大人のキャラメルクッキー",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["新幹線スイーツ", "スイーツ"]),
                Kotoba::from_str("キャラメルクッキー"),
            ],
            fuda: vec!["甘い", "焼き菓子", "メゾンカイザー"],
            neuchi: 430,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/254/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ストーンサークル･クォーター(赤)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("赤ワイン"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 500,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/73/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ストーンサークル･クォーター(白)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("白ワイン"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 500,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/74/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "山崎12年(50ml)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ウイスキー"),
                Kotoba::from_str("山崎"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 1180,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/192/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ジャックダニエルブラック(50ml)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ウイスキー"),
                Kotoba::from_str("ジャックダニエル"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 870,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/191/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "大吟醸カップ(210ml)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("日本酒"),
                Kotoba::from_str("大吟醸カップ"),
            ],
            fuda: vec!["冷たい", "ワンカップ"],
            neuchi: 370,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/219/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ウィルキンソンタンサン缶(250ml)",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("ソフトドリンク"),
                Kotoba::from_vec(vec!["炭酸水", "炭酸飲料", "炭酸"]),
                Kotoba::from_str("ウィルキンソン"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 100,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/189/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "マカデミアナッツ＆さくさくチーズ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["おつまみ", "お摘まみ"]),
                Kotoba::from_str("マカデミアナッツ"),
            ],
            fuda: vec![],
            neuchi: 340,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/71/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "柿の種とピーナッツ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["おつまみ", "お摘まみ"]),
                Kotoba::from_vec(vec!["柿の種", "柿ピー"]),
            ],
            fuda: vec!["ピーナッツ"],
            neuchi: 250,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/25/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "プレミアムミックスナッツ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["おつまみ", "お摘まみ"]),
                Kotoba::from_str("ミックスナッツ"),
            ],
            fuda: vec![],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/27/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "鯛入りちくわ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["おつまみ", "お摘まみ"]),
                Kotoba::from_vec(vec!["ちくわ", "竹輪"]),
            ],
            fuda: vec![],
            neuchi: 370,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/29/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "やわらかビーフジャーキースパイシー味",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["おつまみ", "お摘まみ"]),
                Kotoba::from_str("ビーフジャーキー"),
            ],
            fuda: vec![],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/124/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "しっとりやわらかいか燻製",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["おつまみ", "お摘まみ"]),
                Kotoba::from_vec(vec!["いか燻製", "イカ", "イカ燻製", "鯣", "鯣イカ"]),
            ],
            fuda: vec![],
            neuchi: 370,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/213/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ロッテトッポ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お菓子", "菓子"]),
                Kotoba::from_str("トッポ"),
            ],
            fuda: vec!["ロッテ"],
            neuchi: 260,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/209/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "チップスター極 海の精焼き塩使用しお味",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お菓子", "菓子"]),
                Kotoba::from_str("チップスターしお味"),
            ],
            fuda: vec![
                "ヤマザキ",
                "しお味",
                "チップスター",
                "ポテトチップス",
                "ポテチ",
            ],
            neuchi: 200,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/34/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "チップスター松阪牛ステーキ味",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お菓子", "菓子"]),
                Kotoba::from_str("チップスターステーキ味"),
            ],
            fuda: vec![
                "ヤマザキ",
                "ステーキ味",
                "チップスター",
                "ポテトチップス",
                "ポテチ",
            ],
            neuchi: 200,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/214/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ゆかり濃厚おつまみスナック",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お菓子", "菓子"]),
                Kotoba::from_vec(vec!["おつまみスナック", "お摘まみスナック"]),
            ],
            fuda: vec!["スナック"],
            neuchi: 200,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/151/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "柿もなか2個入",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お菓子", "菓子"]),
                Kotoba::from_vec(vec!["柿もなか", "柿最中", "最中"]),
            ],
            fuda: vec![],
            neuchi: 300,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/212/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "うなぎパイ12本入",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_str("お土産"),
                Kotoba::from_vec(vec!["うなぎパイ", "鰻パイ"]),
            ],
            fuda: vec!["春華堂", "おすすめ"],
            neuchi: 970,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/137/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "崎陽軒　真空パックシウマイ30個入",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_str("お土産"),
                Kotoba::from_vec(vec!["シウマイ", "シュウマイ"]),
            ],
            fuda: vec!["崎陽軒"],
            neuchi: 970,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/46/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "特撰肉づくし",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お弁当", "弁当", "駅弁"]),
                Kotoba::from_str("特撰肉づくし"),
            ],
            fuda: vec![],
            neuchi: 1350,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/264/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "春満喫 鯛めしとたけのこ御膳",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お弁当", "弁当", "駅弁"]),
                Kotoba::from_vec(vec!["たけのこ御膳", "筍御膳"]),
            ],
            fuda: vec![],
            neuchi: 1150,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/279/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "焼売炒飯弁當",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お弁当", "弁当", "駅弁"]),
                Kotoba::from_vec(vec!["焼売炒飯弁当"]),
            ],
            fuda: vec![],
            neuchi: 940,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/259/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "特製幕之内御膳",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お弁当", "弁当", "駅弁"]),
                Kotoba::from_vec(vec!["特製幕之内御膳", "幕の内御膳", "幕の内", "幕内"]),
            ],
            fuda: vec![],
            neuchi: 1380,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/57/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "東海道新幹線弁当",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お弁当", "弁当", "駅弁"]),
                Kotoba::from_str("東海道新幹線弁当"),
            ],
            fuda: vec![],
            neuchi: 1000,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/142/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "厚切りヒレカツサンド",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["サンドイッチ", "サンドウィッチ"]),
                Kotoba::from_str("ヒレカツサンド"),
            ],
            fuda: vec!["ヒレカツ", "カツサンド"],
            neuchi: 750,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/251/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "朝のおむすび弁当",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お弁当", "弁当", "駅弁"]),
                Kotoba::from_vec(vec!["おむすび弁当", "おむすび", "おにぎり"]),
            ],
            fuda: vec![],
            neuchi: 450,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/143/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "牛カルビ焼肉重",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["お弁当", "弁当", "駅弁"]),
                Kotoba::from_vec(vec!["牛カルビ焼肉弁当", "カルビ", "焼肉", "焼き肉"]),
            ],
            fuda: vec![],
            neuchi: 920,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/263/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "プレミアムミックスサンド",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["サンドイッチ", "サンドウィッチ"]),
                Kotoba::from_str("ミックスサンド"),
            ],
            fuda: vec![],
            neuchi: 700,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/127/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "カツ＆ポテトサンド",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["サンドイッチ", "サンドウィッチ"]),
                Kotoba::from_str("カツポテトサンド"),
            ],
            fuda: vec!["カツサンド", "カツポテト"],
            neuchi: 420,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/256/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "ハムチーズ＆たまごサンド",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["サンドイッチ", "サンドウィッチ"]),
                Kotoba::from_str("ハムチーズたまごサンド"),
            ],
            fuda: vec!["ハム", "チーズ", "たまご", "卵", "玉子", "サンド"],
            neuchi: 420,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/129/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "柔らかカツサンド",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["サンドイッチ", "サンドウィッチ"]),
                Kotoba::from_str("カツサンド"),
            ],
            fuda: vec![],
            neuchi: 650,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/267/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "ハムサラダ＆ハムチーズサンド",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("食べ物"),
                Kotoba::from_vec(vec!["サンドイッチ", "サンドウィッチ"]),
                Kotoba::from_str("ハムサラダサンド"),
            ],
            fuda: vec!["ハム", "サラダ", "サンド"],
            neuchi: 470,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/261/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "アイスコーヒー",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("コーヒー"),
                Kotoba::from_vec(vec!["アイスコーヒー", "冷コー"]),
            ],
            fuda: vec!["冷たい"],
            neuchi: 330,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/174/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=4#wagon_content",
        },
        Mono {
            namae: "ホットコーヒー",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("コーヒー"),
                Kotoba::from_str("ホットコーヒー"),
            ],
            fuda: vec!["あたたかい", "暖かい"],
            neuchi: 320,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/157/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=4#wagon_content",
        },
        Mono {
            namae: "サントリー ザ･プレミアムモルツ 350ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ビール"),
                Kotoba::from_str("サントリープレミアムモルツ"),
            ],
            fuda: vec!["冷たい", "サントリー", "モルツ", "プレモル"],
            neuchi: 340,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/1/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "サントリー　ザ・プレミアムモルツ香るエール　350㎖",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ビール"),
                Kotoba::from_str("サントリープレミアムモルツエール"),
            ],
            fuda: vec!["冷たい", "サントリー", "モルツ", "エール"],
            neuchi: 340,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/272/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "キリン一番搾り 350ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ビール"),
                Kotoba::from_vec(vec!["キリン一番搾り", "一番搾り"]),
            ],
            fuda: vec!["冷たい", "キリン"],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/2/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "サッポロエビス 350ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ビール"),
                Kotoba::from_vec(vec!["サッポロエビス", "エビス", "ヱビス"]),
            ],
            fuda: vec!["冷たい", "サッポロ"],
            neuchi: 330,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/5/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "アサヒスーパードライ 350ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ビール"),
                Kotoba::from_vec(vec!["アサヒスーパードライ", "スーパードライ", "極度乾燥"]),
            ],
            fuda: vec!["冷たい", "アサヒ"],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/3/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "サントリー角ハイボール 350ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("ハイボール"),
                Kotoba::from_vec(vec!["サントリー角ハイボール", "ハイボール"]),
            ],
            fuda: vec!["冷たい", "サントリー"],
            neuchi: 270,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/8/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "タカラ缶チューハイレモン 350ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("チューハイ"),
                Kotoba::from_vec(vec!["タカラ缶チューハイレモン", "缶チューハイ"]),
            ],
            fuda: vec!["冷たい", "タカラ"],
            neuchi: 270,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/9/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "こだわり酒場のレモンサワー　500㎖",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_vec(vec!["お酒", "酒", "アルコール"]),
                Kotoba::from_str("サワー"),
                Kotoba::from_str("レモンサワー"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 270,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/280/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "旅茶房静岡茶500ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("ソフトドリンク"),
                Kotoba::from_vec(vec!["お茶", "茶", "緑茶"]),
                Kotoba::from_str("旅茶房静岡茶"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 160,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/253/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "ナチュラルミネラルウォーター Mont Fuji / 500ｍｌ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("ソフトドリンク"),
                Kotoba::from_vec(vec!["ミネラルウェーター", "水", "お水"]),
            ],
            fuda: vec!["冷たい"],
            neuchi: 130,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/20/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "オレンジジュース100 / 280ｍｌ",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("ソフトドリンク"),
                Kotoba::from_str("ジュース"),
                Kotoba::from_str("オレンジジュース"),
            ],
            fuda: vec!["冷たい"],
            neuchi: 180,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/17/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "ポカリスエット/ 300ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("ソフトドリンク"),
                Kotoba::from_str("ジュース"),
                Kotoba::from_vec(vec!["ポカリスエット", "ポカリスウェット", "ポカリ"]),
            ],
            fuda: vec!["冷たい"],
            neuchi: 130,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/16/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "Welch'sグレープ50 280PET",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("ソフトドリンク"),
                Kotoba::from_str("ジュース"),
                Kotoba::from_str("ウェルチグレープ"),
            ],
            fuda: vec!["冷たい", "ウェルチ"],
            neuchi: 140,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/241/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "コカ･コーラ ボトル /280ml",
            category: vec![
                Kotoba::from_str("モノ"),
                Kotoba::from_str("飲み物"),
                Kotoba::from_str("ソフトドリンク"),
                Kotoba::from_str("ジュース"),
                Kotoba::from_vec(vec!["コカコーラ", "コーラ"]),
            ],
            fuda: vec!["冷たい", "炭酸", "炭酸飲料"],
            neuchi: 130,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/15/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
    ];
}
