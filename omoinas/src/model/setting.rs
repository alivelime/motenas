pub struct CotohaToken {
    pub name: String,
    pub date: String,
    pub token: String,
}

impl CotohaToken {
    pub fn date(&self) -> String {
        return self.date.clone();
    }
    pub fn token(&self) -> String {
        return self.token.clone();
    }
}
