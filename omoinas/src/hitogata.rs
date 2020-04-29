pub mod bachan;
pub mod minarai;

pub fn new(name: &str) -> Hitogata {
    return match name {
        "bachan" => bachan::new(),
        "minarai" => minarai::new(),
        _ => bachan::new(),
    };
}

pub struct Hitogata {
    pub id: &'static str,
    pub namae: &'static str,
    pub kaeshi: &'static Kaeshi,
}

pub struct Kaeshi {
    pub aisatsu: Aisatsu,
    pub shirase: Shirase,
    pub toikake: Toikake,
    pub onegai: Onegai,
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
    pub ohayo: AisatsuHibiOhayo,
    pub konnichiwa: AisatsuHibiKonnichiwa,
    pub konbanwa: AisatsuHibiKonbanwa,
    pub oyasumi: AisatsuHibiOyasumi,
}
pub struct AisatsuHibiOhayo {
    pub mayonaka: fn() -> String,
    pub akegata: fn() -> String,
    pub ohayo: fn() -> String,
    pub osoyo: fn() -> String,
    pub ohiru: fn() -> String,
    pub kure: fn() -> String,
    pub yoru: fn() -> String,
}
pub struct AisatsuHibiKonnichiwa {
    pub mayonaka: fn() -> String,
    pub ohayo: fn() -> String,
    pub konnichiwa: fn() -> String,
    pub konbanwa: fn() -> String,
}
pub struct AisatsuHibiKonbanwa {
    pub mayonaka: fn() -> String,
    pub ohayo: fn() -> String,
    pub konnichiwa: fn() -> String,
    pub konbanwa: fn() -> String,
}
pub struct AisatsuHibiOyasumi {
    pub mayonaka: fn() -> String,
    pub ohayo: fn() -> String,
    pub konnichiwa: fn() -> String,
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
    pub desuka: ToikakeDesuka,
}
pub struct ToikakeAru {
    pub iroiro: fn(Vec<String>) -> String,
    pub aru: fn(&str) -> String,
    pub nai: fn(&str) -> String,
    pub wakaran: fn(&str, &str) -> String,
    pub naikedo: fn(&str, &str, &str) -> String,
}
pub struct ToikakeDesuka {
    pub wakaran: fn(&str) -> String,
    pub subete: fn() -> String,
    pub doreka: fn() -> String,
    pub chigau: fn() -> String,
    pub dayo: fn(&str) -> String,
    pub iroiro: fn(Vec<String>) -> String,
    pub naniga: fn() -> String,
    pub ikura: fn(Vec<(&str, u32)>) -> String,
}

pub struct Onegai {
    pub shitai: fn() -> String,
    pub shite: fn() -> String,
    pub shiritai: OnegaiShiritai,
}
pub struct OnegaiShiritai {
    pub kore: fn(&str) -> String,
    pub naniga: fn() -> String,
}
pub struct Tawainai {
    pub ayamaru: fn() -> String,
    pub arigato: fn() -> String,
    pub bochibochi: fn() -> String,
    pub kizukai: fn() -> String,
    pub hagemasu: fn() -> String,
    pub nani: fn() -> String,
    pub ocha: fn(&str, bool) -> String,
    pub sonota: fn() -> String,
    pub sounanda: fn() -> String,
    pub yokatta: fn() -> String,
}

pub struct Wakaran {
    pub wakaran: fn() -> String,
}

pub struct Error {
    pub parse: fn() -> String,
    pub sentence: fn() -> String,
    pub noimpl: fn() -> String,
}
