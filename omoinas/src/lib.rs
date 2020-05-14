pub mod cotoha;
pub mod hitogata;
pub mod model;
pub mod omise;
pub mod omomuki;
pub mod repository;

pub trait Tumori: std::fmt::Debug {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori>;
    fn get_kotae(&self, _: &hitogata::Hitogata) -> omomuki::Result;
}
