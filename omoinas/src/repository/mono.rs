use std::collections::HashSet;

use crate::model::mono::{Mono, MonoResult};
use crate::omomuki;

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
        if self.len() == 0 {
            return Vec::new();
        }
        for i in 0..self.monos[0].category.len() {
            let mut menu = HashSet::new();

            for mono in &self.monos {
                menu.insert(mono.category[i].clone());
            }

            if menu.len() >= 2 {
                return menu.iter().map(|m| m.to_string()).collect();
            }
        }

        return Vec::new();
    }

    pub fn filter_fuda(&self, nani: &String) -> MonoRepository {
        if nani.is_empty() {
            return MonoRepository { monos: Vec::new() };
        }

        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| m.fuda.iter().any(|fuda| fuda.contains(nani)))
                .cloned()
                .collect(),
        };
    }

    pub fn search_category(&self, nani: &String) -> MonoRepository {
        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| {
                    m.namae.contains(nani)
                        || m.category.iter().any(|c| c.contains(nani))
                        || m.fuda.iter().any(|f| f.contains(nani))
                })
                .cloned()
                .collect(),
        };
    }
    pub fn korekana(&self, nani: &Vec<String>) -> MonoRepository {
        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| {
                    m.category.iter().any(|c| c.contains(&nani[0]))
                        || m.fuda.iter().any(|f| f.contains(&nani[0]))
                })
                .cloned()
                .collect(),
        };
    }
    pub fn get_result(&self, nai: &omomuki::Nani, aru: &MonoRepository) -> MonoResult {
        return match self.len() {
            // 絞ったものがなければ、代替案を提示する
            0 => MonoResult::Naikedo(nai.donna_namae(), aru.get_menu()),
            1 | 2 | 3 => MonoResult::Mono(self.monos.iter().map(|m| (*m).clone()).collect()),
            _ => MonoResult::Category(self.get_menu()),
        };
    }
}

pub fn is_mono(nani: &omomuki::Nani) -> bool {
    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());
    let korekana = monos.korekana(&nani.mono);
    if korekana.len() > 0 {
        return true;
    }
    return false;
}
pub fn get_mono(nani: &Option<omomuki::Nani>) -> MonoResult {
    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());
    if let Some(nani) = nani {
        if nani.mono.contains(&String::from("メニュー")) {
            return MonoResult::Category(monos.get_menu());
        }
        // まずカテゴリや商品名・タグで検索
        let namae = nani.namae();
        let searched = monos.search_category(&namae);
        if searched.len() > 0 {
            // 次に形容詞で絞る
            if let Some(donna) = &nani.donna {
                return searched.filter_fuda(donna).get_result(&nani, &searched);
            } else {
                return searched.get_result(&nani, &searched);
            }
        } else {
            if nani.mono.len() >= 2 {
                // なければそれぞの単語で候補を挙げて見る
                let korekana = monos.korekana(&nani.mono);
                if korekana.len() > 0 {
                    return searched.get_result(&nani, &korekana);
                }
            }
            return MonoResult::Nai(nani.namae());
        }
    } else {
        return MonoResult::Category(monos.get_menu());
    }
}

fn get_data() -> Vec<Mono> {
    return vec![
        Mono {
            namae: "スジャータアイスクリーム(バニラ)",
            category: vec![
                "モノ",
                "食べ物",
                "お菓子",
                "アイスクリーム",
                "スジャータ",
                "バニラアイス",
            ],
            fuda: vec!["冷たい", "甘い", "バニラ"],
            neuchi: 290,
            okisa: None,
            aji: Some("バニラ"),
            allergen: Some(vec!["乳", "卵"]),
            calorie: Some(232),
            image: "https://www.jr-cp.co.jp/services/38/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "スジャータアイスクリーム(いちご)",
            category: vec![
                "モノ",
                "食べ物",
                "お菓子",
                "アイスクリーム",
                "スジャータ",
                "ストロベリーアイス",
            ],
            fuda: vec!["冷たい", "甘い", "いちご", "苺", "ストロベリー", "限定"],
            neuchi: 330,
            okisa: None,
            aji: Some("いちご"),
            allergen: Some(vec!["乳"]),
            calorie: Some(162),
            image: "https://www.jr-cp.co.jp/services/281/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "スジャータアイスクリーム(ビスタチオ)",
            category: vec![
                "モノ",
                "食べ物",
                "お菓子",
                "アイスクリーム",
                "スジャータ",
                "ビスタチオアイス",
            ],
            fuda: vec!["冷たい", "甘い", "ビスタチオ", "新しい", "限定"],
            neuchi: 390,
            okisa: None,
            aji: Some("ビスタチオ"),
            allergen: Some(vec!["乳"]),
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/276/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "ウェッジ月刊誌",
            category: vec!["モノ", "読み物", "雑誌", "ウェッジ"],
            fuda: vec!["時代", "先端", "wedge"],
            neuchi: 550,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/50/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "ひととき",
            category: vec!["モノ", "読み物", "雑誌", "ひととき"],
            fuda: vec![],
            neuchi: 550,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/51/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "オーボンヴュータン／プティフールセック",
            category: vec![
                "モノ",
                "食べ物",
                "お菓子",
                "新幹線スイーツ",
                "オーボンヴュータン",
            ],
            fuda: vec!["甘い", "焼き菓子"],
            neuchi: 590,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/276/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "メゾンカイザー　フランボワーズショコラフィナンシェ",
            category: vec![
                "モノ",
                "食べ物",
                "お菓子",
                "新幹線スイーツ",
                "ショコラフィナンシェ",
            ],
            fuda: vec!["甘い", "焼き菓子"],
            neuchi: 540,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/255/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "メゾンカイザー　大人のキャラメルクッキー",
            category: vec![
                "モノ",
                "食べ物",
                "お菓子",
                "新幹線スイーツ",
                "キャラメルクッキー",
            ],
            fuda: vec!["甘い", "焼き菓子"],
            neuchi: 430,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/73/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ストーンサークル･クォーター(赤)",
            category: vec!["モノ", "飲み物", "アルコール", "ワイン", "赤ワイン"],
            fuda: vec!["お酒", "冷たい"],
            neuchi: 500,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/73/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ストーンサークル･クォーター(白)",
            category: vec!["モノ", "飲み物", "アルコール", "ワイン", "白ワイン"],
            fuda: vec!["お酒", "冷たい"],
            neuchi: 500,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/74/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "山崎12年(50ml)",
            category: vec!["モノ", "飲み物", "アルコール", "ウイスキー", "山崎"],
            fuda: vec!["お酒", "冷たい"],
            neuchi: 1180,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/192/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ジャックダニエルブラック(50ml)",
            category: vec![
                "モノ",
                "飲み物",
                "アルコール",
                "ウイスキー",
                "ジャックダニエル",
            ],
            fuda: vec!["お酒", "冷たい"],
            neuchi: 870,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/191/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "大吟醸カップ(210ml)",
            category: vec!["モノ", "飲み物", "アルコール", "日本酒", "大吟醸"],
            fuda: vec!["お酒", "冷たい", "ワンカップ"],
            neuchi: 370,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/219/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ウィルキンソンタンサン缶(250ml)",
            category: vec!["モノ", "飲み物", "ソフトドリンク", "炭酸", "ウィルキンソン"],
            fuda: vec!["冷たい", "甘くない"],
            neuchi: 100,
            okisa: None,
            aji: None,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/189/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
    ];
}
