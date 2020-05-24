pub mod application;
pub mod model;
pub mod omomuki;
pub mod repository;
pub mod service;

use crate::model::hitogata::Hitogata;

pub trait Tumori: std::fmt::Debug {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori>;
    fn get_kotae(&self, _: &Hitogata) -> omomuki::Result;
}
