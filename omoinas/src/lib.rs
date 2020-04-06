pub mod cotoha;
pub mod kaeshi;
pub mod model;
pub mod omomuki;

pub trait Tumori {
    fn get_kotae(&self) -> String;
}
