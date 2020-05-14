use crate::model::{Kotoba, Nani};

pub enum MonoResult {
    Category(Vec<String>),
    Mono(Vec<Mono>, Vec<String>),
    Naikedo(String, String, Vec<Mono>, Vec<String>),
    Nai(Nani),
}

pub enum Desu {
    Wakaran(String),
    Subete(),
    Doreka(),
    Chigau(),
    IsCategory(String),
    Category(Vec<String>),
    Mono(Vec<Mono>, Vec<String>),
    Ikura(Vec<Mono>),
}

#[derive(Clone, Debug)]
pub struct Mono {
    pub namae: &'static str,
    pub category: Vec<Kotoba>,
    pub fuda: Vec<&'static str>,
    pub neuchi: u32,
    pub comment: &'static str,
    pub allergen: Option<Vec<&'static str>>,
    pub ingredients: Option<Vec<&'static str>>,
    pub calorie: Option<u32>,
    pub image: &'static str,
    pub url: &'static str,
}
