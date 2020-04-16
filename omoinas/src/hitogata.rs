pub mod bachan;
pub mod minarai;

pub fn new(name: &str) -> Hitogata {
    return match name {
        "bachan" => bachan::new(),
        // "minarai" => minarai::new(),
        _ => bachan::new(),
    };
}

pub struct Hitogata {
    pub kaeshi: &'static Kaeshi,
}

pub struct Kaeshi {
    pub aisatsu: Aisatsu,
    pub shirase: Shirase,
    pub toikake: Toikake,
    pub tawainai: Tawainai,
    pub wakaran: Wakaran,
    pub kitanai: fn(&str) -> String,
    pub error: Error,
}
pub struct Aisatsu {
    pub hibi: AisatsuHibi,
    pub iku: AisatsuIku,
    pub kuru: AisatsuKuru,
}
pub struct AisatsuHibi {
    pub ohayo: fn() -> String,
    pub konnichiwa: fn() -> String,
    pub konbanwa: fn() -> String,
    pub oyasumi: fn() -> String,
}
pub struct AisatsuIku {
    pub matane: AisatsuIkuMatane,
    pub sayonara: fn() -> String,
    pub ittera: AisatsuIkuIttera,
}
pub struct AisatsuIkuMatane {
    pub toki: fn(&str) -> String,
    pub tokoro: fn(&str) -> String,
    pub mata: fn() -> String,
}
pub struct AisatsuIkuIttera {
    pub toki: fn(&str) -> String,
    pub tokoro: fn(&str) -> String,
    pub tokitokoro: fn(&str, &str) -> String,
    pub ittera: fn() -> String,
}

pub struct AisatsuKuru {
    pub hajimemashite: fn() -> String,
    pub hisashiburi: fn() -> String,
    pub okaeri: fn() -> String,
    pub irasshai: fn() -> String,
}

pub struct Shirase {
    pub iku: ShiraseIku,
    pub kuru: ShiraseKuru,
}
pub struct ShiraseIku {}
pub struct ShiraseKuru {}

pub struct Toikake {
    pub aru: ToikakeAru,
}
pub struct ToikakeAru {
    pub iroiro: fn(Vec<String>) -> String,
    pub aru: fn(&str) -> String,
    pub nai: fn(&str) -> String,
    pub wakaran: fn(&str, &str) -> String,
    pub naikedo: fn(&str, &str) -> String,
}

pub struct Tawainai {
    pub nani: fn() -> String,
}

pub struct Wakaran {
    pub wakaran: fn() -> String,
}

pub struct Error {
    pub parse: fn() -> String,
    pub sentence: fn() -> String,
    pub noimpl: fn() -> String,
}
