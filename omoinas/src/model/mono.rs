use crate::model;

pub enum MonoResult {
    Category(Vec<String>),
    Mono(Vec<Mono>),
    Naikedo(String, String, Vec<String>),
    Nai(model::Nani),
}

pub enum Desu {
    Wakaran(String),
    Subete(),
    Doreka(),
    Chigau(),
    IsCategory(String),
    Category(Vec<String>),
    Mono(Vec<Mono>),
    Ikura(Vec<Mono>),
}

#[derive(Clone, Debug)]
pub struct Mono {
    pub namae: &'static str,
    pub category: Vec<&'static str>,
    pub fuda: Vec<&'static str>,
    pub neuchi: u32,
    //    pub okisa: Option<(u32, &'static str)>,
    //    pub aji: Option<&'static str>,
    pub allergen: Option<Vec<&'static str>>,
    pub calorie: Option<u32>,
    pub image: &'static str,
    pub url: &'static str,
}
