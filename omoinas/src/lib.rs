pub mod cotoha;
pub mod hitogata;
pub mod model;
pub mod omomuki;
pub mod repository;

pub trait Tumori: std::fmt::Debug {
    fn kotafu(&self) -> Box<dyn Tumori>;
    fn get_kotae(&self, _: &hitogata::Hitogata) -> String;
}
