use crate::Tumori;

pub struct Okaeri {}

impl Tumori for Okaeri {
    fn get_kotae(&self) -> String {
        return String::from("はい、おかえり");
    }
}
