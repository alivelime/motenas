use std::collections::HashSet;

use crate::model::mono::{Desu, Mono, MonoResult};
use crate::model::{Koto, Nani};
use crate::omise::Omise;

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
    pub fn menu(&self) -> Vec<String> {
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
    pub fn result(&self, nai: Option<(String, String)>) -> MonoResult {
        return if let Some((nai, nara)) = nai {
            // 絞ったものがなければ、代替案を提示する
            MonoResult::Naikedo(nai.clone(), nara.clone(), self.mono(), self.menu())
        } else {
            MonoResult::Mono(self.mono(), self.menu())
        };
    }
    pub fn category(&self, nani: &String) -> Option<String> {
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
    pub fn mono(&self) -> Vec<Mono> {
        return self.monos.iter().map(|m| (*m).clone()).collect();
    }
}

pub fn is_mono(omise: &Omise, nani: &Nani) -> bool {
    let monos = MonoRepository::new(omise.menu());
    let korekana = monos.korekana(&nani.mono);
    if korekana.len() > 0 {
        return true;
    }
    return false;
}
pub fn get_mono(omise: &Omise, nani: Option<&Nani>) -> MonoResult {
    let monos = MonoRepository::new(omise.menu());
    if let Some(nani) = nani {
        // 冷たいチョコアイス?
        if let Some(donna) = &nani.donna {
            let searched = monos.match_all([nani.mono.clone(), vec![donna.clone()]].concat());
            if searched.len() > 0 {
                return searched.result(None);
            }
        }

        // チョコアイス?
        let searched = monos.match_all(nani.mono.clone());
        if searched.len() > 0 {
            return if nani.donna.is_some() {
                searched.result(Some((nani.donna_namae(), nani.namae())))
            } else {
                searched.result(None)
            };
        }

        // アイス?
        if nani.mono.len() >= 2 {
            let korekana = monos.korekana(&nani.mono);
            if korekana.len() > 0 {
                return korekana.result(Some((nani.namae(), nani.mono[0].to_string())));
            }
        }

        return MonoResult::Nai(nani.clone());
    } else {
        return MonoResult::Category(monos.menu());
    }
}

pub fn zakkuri(omise: &Omise, kore: &Nani) -> Desu {
    let monos = MonoRepository::new(omise.menu());
    if let Some(category) = monos.category(&kore.namae()) {
        return Desu::IsCategory(category);
    }
    return match get_mono(omise, Some(&kore)) {
        MonoResult::Category(category) => Desu::Category(category),
        MonoResult::Mono(mono, category) => Desu::Mono(mono, category),
        MonoResult::Naikedo(nai, _, _, _) => Desu::Wakaran(nai),
        MonoResult::Nai(mono) => Desu::Wakaran(mono.donna_namae()),
    };
}
pub fn ikura(omise: &Omise, kore: &Nani) -> Desu {
    return match get_mono(omise, Some(&kore)) {
        MonoResult::Category(category) => Desu::Category(category),
        MonoResult::Mono(mono, _) => Desu::Ikura(mono),
        MonoResult::Naikedo(nai, _, _, _) => Desu::Wakaran(nai),
        MonoResult::Nai(mono) => Desu::Wakaran(mono.donna_namae()),
    };
}

pub fn desuka(omise: &Omise, kore: &Nani, are: &Nani) -> Desu {
    if kore.donna_namae() == are.donna_namae() {
        return Desu::Subete();
    }

    if are.mono.len() > 0 && are.has(vec!["何"]) {
        return zakkuri(omise, kore);
    }
    if are.mono.len() > 0 && are.has(vec!["幾等", "おいくら"]) {
        return ikura(omise, kore);
    }

    let monos = MonoRepository::new(omise.menu());

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
