use crate::model::mono::Mono;

pub struct Client {
    pub id: &'static str,
    pub namae: &'static str,
    pub hp: &'static str,
}
pub struct Omise {
    pub id: String,
    pub client: Client,
    pub namae: String,
    pub hp: String,
    pub yotei: String,
    pub menu: Vec<Mono>,
    pub status: Status,

    pub created_at: String,
}

impl Omise {
    pub fn menu(&self) -> Vec<&Mono> {
        return self.menu.iter().collect::<Vec<&Mono>>();
    }
}

#[derive(Clone, Debug)]
pub enum Status {
    Yasumi,
    Hima,
    Bochibochi,
    Isogashi,
    Kashikiri,
}

/*
pub struct Address {
    namae: String,
    country: String,
    postcode: u32,
    prefcode: u32,
    city: String,
    address1: String,
    address2: String,
    access: String,
    lat: f64,
    lng: f64,
}
*/
