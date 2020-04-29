use std::collections::HashSet;

use crate::model;
use crate::model::mono::{Desu, Mono, MonoResult};

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
                            menu.insert(mono.category[i].clone());
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

    pub fn filter_fuda(&self, nani: &String) -> MonoRepository {
        if nani.is_empty() {
            return MonoRepository { monos: Vec::new() };
        }

        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| m.fuda.contains(&nani.as_str()))
                .cloned()
                .collect(),
        };
    }

    pub fn match_all(&self, nani: Vec<String>) -> MonoRepository {
        let namae = nani.iter().rev().cloned().collect::<Vec<String>>().join("");
        return MonoRepository {
            monos: self
                .monos
                .iter()
                .filter(|&m| {
                    nani.iter()
                        .all(|n| m.category.contains(&n.as_str()) || m.fuda.contains(&n.as_str()))
                        || m.category.contains(&namae.as_str())
                        || m.fuda.contains(&namae.as_str())
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
                    m.category.contains(&nani[0].as_str()) || m.fuda.contains(&nani[0].as_str())
                })
                .cloned()
                .collect(),
        };
    }
    pub fn get_result(&self, nai: Option<(&String, &String)>) -> MonoResult {
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
            if let Some(p) = m.category.iter().position(|&c| c == nani) {
                Some(m.category[p - 1].to_string())
            } else {
                None
            }
        });
    }
    pub fn include_all(&self, kore: &Vec<String>) -> bool {
        return self.monos.iter().all(|m| {
            kore.iter()
                .all(|k| m.category.contains(&k.as_str()) || m.fuda.contains(&k.as_str()))
        });
    }
    pub fn include_any(&self, kore: &Vec<String>) -> bool {
        return self.monos.iter().any(|m| {
            kore.iter()
                .all(|k| m.category.contains(&k.as_str()) || m.fuda.contains(&k.as_str()))
        });
    }
}

pub fn is_mono(nani: &model::Nani) -> bool {
    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());
    let korekana = monos.korekana(&nani.mono);
    if korekana.len() > 0 {
        return true;
    }
    return false;
}
pub fn get_mono(nani: &Option<model::Nani>) -> MonoResult {
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
                searched.get_result(Some((&nani.donna_namae(), &nani.namae())))
            } else {
                searched.get_result(None)
            };
        }

        // アイス?
        if nani.mono.len() >= 2 {
            let korekana = monos.korekana(&nani.mono);
            if korekana.len() > 0 {
                return korekana.get_result(Some((&nani.namae(), &nani.mono[0])));
            }
        }

        return MonoResult::Nai(nani.clone());
    } else {
        return MonoResult::Category(monos.get_menu());
    }
}

pub fn zakkuri(kore: &model::Nani) -> Desu {
    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());
    if let Some(category) = monos.get_category(&kore.namae()) {
        return Desu::IsCategory(category);
    }
    return match get_mono(&Some(kore.clone())) {
        MonoResult::Category(category) => Desu::Category(category),
        MonoResult::Mono(mono) => Desu::Mono(mono),
        MonoResult::Naikedo(nai, _, _) => Desu::Wakaran(nai),
        MonoResult::Nai(mono) => Desu::Wakaran(mono.donna_namae()),
    };
}
pub fn ikura(kore: &model::Nani) -> Desu {
    return match get_mono(&Some(kore.clone())) {
        MonoResult::Category(category) => Desu::Category(category),
        MonoResult::Mono(mono) => Desu::Ikura(mono),
        MonoResult::Naikedo(nai, _, _) => Desu::Wakaran(nai),
        MonoResult::Nai(mono) => Desu::Wakaran(mono.donna_namae()),
    };
}

pub fn desuka(kore: &model::Nani, are: &model::Nani) -> Desu {
    if kore.donna_namae() == are.donna_namae() {
        return Desu::Subete();
    }

    if are.mono.len() > 0 && are.mono[0].as_str() == "何" {
        return zakkuri(kore);
    }
    if are.mono.len() > 0 && vec!["幾等", "おいくら"].contains(&are.mono[0].as_str()) {
        return ikura(kore);
    }

    let data = get_data();
    let monos = MonoRepository::new(data.iter().collect::<Vec<&Mono>>());

    // 冷たいチョコアイス?
    let searched = monos.match_all(match &kore.donna {
        Some(donna) => [kore.mono.clone(), vec![donna.clone()]].concat(),
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
            category: vec!["モノ", "メニュー"],
            fuda: vec![],
            neuchi: 0,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/topics/get_image?id=215&image_number=1",
            url: "https://www.jr-cp.co.jp/pdf/service/wagon_menu_list.pdf",
        },
        Mono {
            namae: "スジャータアイスクリーム(バニラ)",
            category: vec!["モノ", "食べ物", "アイスクリーム", "バニラアイス"],
            fuda: vec!["冷たい", "甘い", "バニラ", "アイス", "スジャータ"],
            neuchi: 290,
            allergen: Some(vec!["乳", "卵"]),
            calorie: Some(232),
            image: "https://www.jr-cp.co.jp/services/38/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "スジャータアイスクリーム(いちご)",
            category: vec!["モノ", "食べ物", "アイスクリーム", "ストロベリーアイス"],
            fuda: vec![
                "冷たい",
                "甘い",
                "苺",
                "ストロベリー",
                "アイス",
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
            category: vec!["モノ", "食べ物", "アイスクリーム", "ピスタチオアイス"],
            fuda: vec![
                "冷たい",
                "甘い",
                "ピスタチオ",
                "アイス",
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
            category: vec!["モノ", "読み物", "雑誌", "ウェッジ"],
            fuda: vec!["時代の先端を行く雑誌", "wedge"],
            neuchi: 550,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/50/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "ひととき",
            category: vec!["モノ", "読み物", "雑誌", "ひととき"],
            fuda: vec![],
            neuchi: 550,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/51/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=1#wagon_content",
        },
        Mono {
            namae: "オーボンヴュータン／プティフールセック",
            category: vec!["モノ", "食べ物", "新幹線スイーツ", "プティフールセック"],
            fuda: vec!["甘い", "焼き菓子", "オーボンヴュータン"],
            neuchi: 590,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/41/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "メゾンカイザー　フランボワーズショコラフィナンシェ",
            category: vec!["モノ", "食べ物", "新幹線スイーツ", "ショコラフィナンシェ"],
            fuda: vec!["甘い", "焼き菓子", "メゾンカイザー"],
            neuchi: 540,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/255/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "メゾンカイザー　大人のキャラメルクッキー",
            category: vec!["モノ", "食べ物", "新幹線スイーツ", "キャラメルクッキー"],
            fuda: vec!["甘い", "焼き菓子", "メゾンカイザー"],
            neuchi: 430,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/254/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ストーンサークル･クォーター(赤)",
            category: vec!["モノ", "飲み物", "お酒", "ワイン", "赤ワイン"],
            fuda: vec!["冷たい", "アルコール"],
            neuchi: 500,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/73/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ストーンサークル･クォーター(白)",
            category: vec!["モノ", "飲み物", "お酒", "ワイン", "白ワイン"],
            fuda: vec!["冷たい", "アルコール"],
            neuchi: 500,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/74/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "山崎12年(50ml)",
            category: vec!["モノ", "飲み物", "お酒", "ウイスキー", "山崎"],
            fuda: vec!["冷たい", "アルコール"],
            neuchi: 1180,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/192/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ジャックダニエルブラック(50ml)",
            category: vec!["モノ", "飲み物", "お酒", "ウイスキー", "ジャックダニエル"],
            fuda: vec!["冷たい", "アルコール"],
            neuchi: 870,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/191/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "大吟醸カップ(210ml)",
            category: vec!["モノ", "飲み物", "お酒", "日本酒", "大吟醸カップ"],
            fuda: vec!["冷たい", "ワンカップ", "アルコール"],
            neuchi: 370,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/219/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ウィルキンソンタンサン缶(250ml)",
            category: vec![
                "モノ",
                "飲み物",
                "ソフトドリンク",
                "炭酸飲料",
                "ウィルキンソン",
            ],
            fuda: vec!["冷たい", "炭酸", "炭酸水"],
            neuchi: 100,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/189/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "マカデミアナッツ＆さくさくチーズ",
            category: vec!["モノ", "食べ物", "おつまみ", "マカデミアナッツ"],
            fuda: vec!["マカデミアナッツ", "お摘まみ"],
            neuchi: 340,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/71/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "柿の種とピーナッツ",
            category: vec!["モノ", "食べ物", "おつまみ", "柿の種"],
            fuda: vec!["ピーナッツ", "柿ピー", "お摘まみ"],
            neuchi: 250,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/25/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "プレミアムミックスナッツ",
            category: vec!["モノ", "食べ物", "おつまみ", "ミックスナッツ"],
            fuda: vec!["お摘まみ"],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/27/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "鯛入りちくわ",
            category: vec!["モノ", "食べ物", "おつまみ", "ちくわ"],
            fuda: vec!["お摘まみ"],
            neuchi: 370,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/29/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "やわらかビーフジャーキースパイシー味",
            category: vec!["モノ", "食べ物", "おつまみ", "ビーフジャーキー"],
            fuda: vec!["お摘まみ"],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/124/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "しっとりやわらかいか燻製",
            category: vec!["モノ", "食べ物", "おつまみ", "いか"],
            fuda: vec!["鯣", "鯣イカ", "お摘まみ"],
            neuchi: 370,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/213/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "ロッテトッポ",
            category: vec!["モノ", "食べ物", "菓子", "トッポ"],
            fuda: vec!["ロッテ"],
            neuchi: 260,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/209/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "チップスター極 海の精焼き塩使用しお味",
            category: vec!["モノ", "食べ物", "菓子", "チップスターしお味"],
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
            category: vec!["モノ", "食べ物", "菓子", "チップスターステーキ味"],
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
            category: vec!["モノ", "食べ物", "菓子", "おつまみスナック"],
            fuda: vec!["スナック"],
            neuchi: 200,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/151/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "柿もなか2個入",
            category: vec!["モノ", "食べ物", "菓子", "柿もなか"],
            fuda: vec!["最中", "モナカ"],
            neuchi: 300,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/212/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "うなぎパイ12本入",
            category: vec!["モノ", "食べ物", "お土産", "うなぎパイ"],
            fuda: vec!["春華堂", "おすすめ", "鰻パイ"],
            neuchi: 970,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/137/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "崎陽軒　真空パックシウマイ30個入",
            category: vec!["モノ", "食べ物", "お土産", "シウマイ"],
            fuda: vec!["崎陽軒"],
            neuchi: 970,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/46/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=2#wagon_content",
        },
        Mono {
            namae: "特撰肉づくし",
            category: vec!["モノ", "食べ物", "弁当", "特撰肉づくし"],
            fuda: vec!["駅弁"],
            neuchi: 1350,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/264/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "春満喫 鯛めしとたけのこ御膳",
            category: vec!["モノ", "食べ物", "弁当", "たけのこ御膳"],
            fuda: vec!["駅弁", "筍御膳"],
            neuchi: 1150,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/279/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "焼売炒飯弁當",
            category: vec!["モノ", "食べ物", "弁当", "焼売炒飯弁當"],
            fuda: vec!["駅弁"],
            neuchi: 940,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/259/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "特製幕之内御膳",
            category: vec!["モノ", "食べ物", "弁当", "特製幕之内御膳"],
            fuda: vec!["駅弁", "幕の内弁当", "幕内"],
            neuchi: 1380,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/57/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "東海道新幹線弁当",
            category: vec!["モノ", "食べ物", "弁当", "東海道新幹線弁当"],
            fuda: vec!["駅弁"],
            neuchi: 1000,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/142/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "厚切りヒレカツサンド",
            category: vec!["モノ", "食べ物", "サンドイッチ", "ヒレカツサンド"],
            fuda: vec!["ヒレカツ", "カツサンド"],
            neuchi: 750,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/251/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "朝のおむすび弁当",
            category: vec!["モノ", "食べ物", "弁当", "おむすび弁当"],
            fuda: vec!["駅弁", "おむすび", "おにぎり"],
            neuchi: 450,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/143/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "牛カルビ焼肉重",
            category: vec!["モノ", "食べ物", "弁当", "牛カルビ焼肉重"],
            fuda: vec!["駅弁", "焼肉", "牛カルビ", "カルビ", "焼き肉"],
            neuchi: 920,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/263/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "プレミアムミックスサンド",
            category: vec!["モノ", "食べ物", "サンドイッチ", "ミックスサンド"],
            fuda: vec![],
            neuchi: 700,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/127/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "カツ＆ポテトサンド",
            category: vec!["モノ", "食べ物", "サンドイッチ", "カツポテトサンド"],
            fuda: vec!["カツサンド", "カツポテト"],
            neuchi: 420,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/256/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "ハムチーズ＆たまごサンド",
            category: vec!["モノ", "食べ物", "サンドイッチ", "ハムチーズたまごサンド"],
            fuda: vec!["ハム", "チーズ", "たまご"],
            neuchi: 420,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/129/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "柔らかカツサンド",
            category: vec!["モノ", "食べ物", "サンドイッチ", "カツサンド"],
            fuda: vec![],
            neuchi: 650,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/267/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "ハムサラダ＆ハムチーズサンド",
            category: vec!["モノ", "食べ物", "サンドイッチ", "ハムサラダサンド"],
            fuda: vec!["ハム", "チーズ"],
            neuchi: 470,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/261/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=3#wagon_content",
        },
        Mono {
            namae: "アイスコーヒー",
            category: vec!["モノ", "飲み物", "コーヒー", "アイスコーヒー"],
            fuda: vec!["冷たい"],
            neuchi: 330,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/174/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=4#wagon_content",
        },
        Mono {
            namae: "ホットコーヒー",
            category: vec!["モノ", "飲み物", "コーヒー", "ホットコーヒー"],
            fuda: vec!["あたたかい"],
            neuchi: 320,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/157/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=4#wagon_content",
        },
        Mono {
            namae: "サントリー ザ･プレミアムモルツ 350ml",
            category: vec![
                "モノ",
                "飲み物",
                "お酒",
                "ビール",
                "サントリープレミアムモルツ",
            ],
            fuda: vec!["冷たい", "サントリー", "モルツ", "プレモル", "アルコール"],
            neuchi: 340,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/1/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "サントリー　ザ・プレミアムモルツ香るエール　350㎖",
            category: vec![
                "モノ",
                "飲み物",
                "お酒",
                "ビール",
                "サントリープレミアムモルツエール",
            ],
            fuda: vec!["冷たい", "サントリー", "モルツ", "エール", "アルコール"],
            neuchi: 340,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/272/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "キリン一番搾り 350ml",
            category: vec!["モノ", "飲み物", "お酒", "ビール", "キリン一番搾り"],
            fuda: vec!["冷たい", "キリン", "アルコール"],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/2/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "サッポロエビス 350ml",
            category: vec!["モノ", "飲み物", "お酒", "ビール", "サッポロエビス"],
            fuda: vec!["冷たい", "サッポロ", "エビス", "アルコール"],
            neuchi: 330,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/5/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "アサヒスーパードライ 350ml",
            category: vec!["モノ", "飲み物", "お酒", "ビール", "アサヒスーパードライ"],
            fuda: vec![
                "冷たい",
                "アサヒ",
                "スーパードライ",
                "極度乾燥",
                "アルコール",
            ],
            neuchi: 310,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/3/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "サントリー角ハイボール 350ml",
            category: vec![
                "モノ",
                "飲み物",
                "お酒",
                "ハイボール",
                "サントリー角ハイボール",
            ],
            fuda: vec!["冷たい", "サントリー", "角ハイボール", "アルコール"],
            neuchi: 270,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/8/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "タカラ缶チューハイレモン 350ml",
            category: vec![
                "モノ",
                "飲み物",
                "お酒",
                "チューハイ",
                "タカラ缶チューハイレモン",
            ],
            fuda: vec!["冷たい", "タカラ", "アルコール"],
            neuchi: 270,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/9/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "こだわり酒場のレモンサワー　500㎖",
            category: vec!["モノ", "飲み物", "お酒", "サワー", "レモンサワー"],
            fuda: vec!["冷たい", "アルコール"],
            neuchi: 270,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/280/get_image?image_number=1<Paste>",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "旅茶房静岡茶500ml",
            category: vec!["モノ", "飲み物", "ソフトドリンク", "お茶", "旅茶房静岡茶"],
            fuda: vec!["冷たい", "緑茶"],
            neuchi: 160,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/253/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "ナチュラルミネラルウォーター Mont Fuji / 500ｍｌ",
            category: vec![
                "モノ",
                "飲み物",
                "ソフトドリンク",
                "水",
                "ミネラルウォーター",
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
                "モノ",
                "飲み物",
                "ソフトドリンク",
                "ジュース",
                "オレンジジュース",
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
            category: vec!["モノ", "飲み物", "ソフトドリンク", "ポカリスエット/ 300ml"],
            fuda: vec!["冷たい", "ポカリスウェット"],
            neuchi: 130,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/16/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
        Mono {
            namae: "Welch'sグレープ50 280PET",
            category: vec![
                "モノ",
                "飲み物",
                "ソフトドリンク",
                "ジュース",
                "ウェルチグレープ",
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
            category: vec!["モノ", "飲み物", "ソフトドリンク", "コカコーラ"],
            fuda: vec!["冷たい", "コーラ"],
            neuchi: 130,
            allergen: None,
            calorie: None,
            image: "https://www.jr-cp.co.jp/services/15/get_image?image_number=1",
            url: "https://www.jr-cp.co.jp/services/wagon?space=5#wagon_content",
        },
    ];
}
