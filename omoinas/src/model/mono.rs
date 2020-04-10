pub enum MonoResult {
    Category(Vec<&'static str>),
    Mono(Vec<Mono>),
    Kawari(String, Vec<Mono>),
    Wakaran(String, String),
    Nai(String),
}

pub enum MonoType {
    Tsumetai,
    Atatakai,

    Other,
}

pub struct Mono {
    pub name: String,
    pub donna: MonoType,
}
