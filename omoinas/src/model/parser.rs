use crate::model::cache::Cache;
use crate::model::kotoba::{Koto, Nani};
use crate::model::omomuki::{Dearu, Keiyou, Suru, Taigen};

pub trait Parser {
    fn parse<C: Cache>(text: &String) -> (bool, Self); // Result<Self, ..>が error(E0277)になる
    fn get_access_token() -> Result<String, reqwest::Error>;

    fn is_hatena(&self) -> bool;
    fn is_yonda(&self) -> bool;
    fn is_kitanai(&self) -> Option<String>;
    fn get_doushi(&self) -> Option<(Suru, bool, bool)>;
    fn get_keidou(&self) -> Option<Keiyou>;
    fn get_taigen(&self) -> Option<Taigen>;
    fn get_kore_are(&self) -> Option<Dearu>;
    fn get_meishi(&self) -> Vec<Nani>;

    fn get_kotoba(&self) -> Vec<Koto>;
    fn get_hitokoto(&self) -> Option<Koto>;
    fn get_odoroki(&self) -> Option<Koto>;

    fn get_itsu(&self) -> Option<Koto>;
    fn get_doko(&self) -> Option<Koto>;

    fn has_lemma(&self, p: Vec<&str>) -> Option<String>;
}
