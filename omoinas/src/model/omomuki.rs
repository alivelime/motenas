use crate::model::kotoba::*;

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

#[derive(Clone, Debug)]
pub struct Ocha {
    pub nani: Vec<Nani>,
    pub hatena: bool,
}

#[derive(Clone, Debug)]
pub struct Tawainai {
    pub itsu: Option<Kotoba>,
    pub doko: Option<Kotoba>,
    pub hatena: bool,
}
