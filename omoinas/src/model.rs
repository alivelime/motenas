pub mod mono;
pub mod setting;

use std::cmp::PartialEq;

#[derive(Clone, Debug)]
pub struct Koto {
    pub yomi: String,
    pub moji: String,
}
impl Koto {
    pub fn new<T1, T2>(y: T1, m: T2) -> Koto
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        return Koto {
            yomi: y.into(),
            moji: m.into(),
        };
    }
    pub fn from_str<T>(m: T) -> Koto
    where
        T: Into<String>,
    {
        return Koto {
            yomi: String::from(""),
            moji: m.into(),
        };
    }
    pub fn as_str(&self) -> &str {
        return self.moji.as_str();
    }
    pub fn to_string(&self) -> String {
        return self.moji.clone();
    }
}

#[derive(Clone, Debug)]
pub struct Kotoba {
    pub w: Vec<Koto>,
}
impl Kotoba {
    pub fn new(k: Vec<Koto>) -> Kotoba {
        return Kotoba { w: k };
    }
    pub fn from_vec(w: Vec<&str>) -> Kotoba {
        return Kotoba::new(
            w.into_iter()
                .map(|m| Koto::new("", m))
                .collect::<Vec<Koto>>(),
        );
    }
    pub fn from_str(w: &str) -> Kotoba {
        return Kotoba {
            w: vec![Koto::new("", w)],
        };
    }
    pub fn from(k: Koto) -> Kotoba {
        return Kotoba { w: vec![k] };
    }
    pub fn as_str(&self) -> &str {
        return self.w[0].as_str();
    }
    pub fn to_string(&self) -> String {
        return self.w[0].to_string();
    }
}

impl PartialEq for Koto {
    fn eq(&self, other: &Self) -> bool {
        return self.moji == other.moji;
    }
}
impl PartialEq<str> for Koto {
    fn eq(&self, other: &str) -> bool {
        return self.moji == other;
    }
}
impl PartialEq<Koto> for str {
    fn eq(&self, other: &Koto) -> bool {
        return self == other.moji;
    }
}
impl PartialEq<&str> for Koto {
    fn eq(&self, other: &&str) -> bool {
        return &self.moji == other;
    }
}
impl PartialEq<Koto> for &str {
    fn eq(&self, other: &Koto) -> bool {
        return self == &other.moji;
    }
}
impl PartialEq<String> for Koto {
    fn eq(&self, other: &String) -> bool {
        return &self.moji == other;
    }
}
impl PartialEq<Koto> for String {
    fn eq(&self, other: &Koto) -> bool {
        return self == &other.moji;
    }
}

impl PartialEq for Kotoba {
    fn eq(&self, other: &Self) -> bool {
        return self
            .w
            .iter()
            .any(|w| other.w.iter().any(|o| w.moji == o.moji));
    }
}
impl PartialEq<str> for Kotoba {
    fn eq(&self, other: &str) -> bool {
        return self.w.iter().any(|w| &w.moji == other);
    }
}
impl PartialEq<Kotoba> for str {
    fn eq(&self, other: &Kotoba) -> bool {
        return other.w.iter().any(|w| &w.moji == self);
    }
}
impl PartialEq<&str> for Kotoba {
    fn eq(&self, other: &&str) -> bool {
        return self.w.iter().any(|w| &w.moji == other);
    }
}
impl PartialEq<Kotoba> for &str {
    fn eq(&self, other: &Kotoba) -> bool {
        return other.w.iter().any(|w| &w.moji == self);
    }
}
impl PartialEq<String> for Kotoba {
    fn eq(&self, other: &String) -> bool {
        return self.w.iter().any(|k| &k.moji == other);
    }
}
impl PartialEq<Kotoba> for String {
    fn eq(&self, other: &Kotoba) -> bool {
        return other.w.iter().any(|w| &w.moji == self);
    }
}
impl PartialEq<Kotoba> for Koto {
    fn eq(&self, other: &Kotoba) -> bool {
        return other.w.iter().any(|w| w.moji == self.moji);
    }
}
impl PartialEq<Koto> for Kotoba {
    fn eq(&self, other: &Koto) -> bool {
        return self.w.iter().any(|w| w.moji == other.moji);
    }
}

#[derive(Clone, Debug)]
pub struct Nani {
    pub donna: Option<Koto>,
    pub mono: Vec<Koto>,
}
impl Nani {
    pub fn donna_namae(&self) -> String {
        return format!(
            "{}{}",
            match &self.donna {
                Some(donna) => donna.as_str(),
                None => "",
            },
            self.namae()
        );
    }
    pub fn namae(&self) -> String {
        return self
            .mono
            .iter()
            .rev()
            .map(|k| k.as_str().to_string())
            .collect::<Vec<String>>()
            .join("");
    }
    pub fn has(&self, w: Vec<&str>) -> bool {
        return self.mono.iter().any(|k| w.contains(&k.moji.as_str()));
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Toki {
    Nochi,
    Ima,
    Mukashi,
}
#[derive(Clone, Debug)]
pub struct Doushita {
    pub ina: bool,
    pub toki: Toki,
    pub ukemi: bool,
    pub suru: Koto,
}

#[derive(Clone, Debug)]
pub struct Suru {
    pub itsu: Option<Koto>,
    pub doko: Option<Koto>,
    pub dare: Option<Koto>,
    pub nani: Vec<Nani>,
    pub doushita: Doushita,
    pub hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Taigen {
    pub itsu: Option<Koto>,
    pub doko: Option<Koto>,
    pub nani: Vec<Nani>,
    pub suru: Koto,
    pub hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Keiyou {
    pub itsu: Option<Koto>,
    pub doko: Option<Koto>,
    pub dare: Option<Koto>,
    pub nani: Vec<Nani>,
    pub dou: Koto,
    pub ina: bool,
    pub toki: Toki,
    pub hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Dearu {
    pub kore: Nani,
    pub are: Nani,
    pub hatena: bool,
}
