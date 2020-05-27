pub struct ParserToken {
    pub name: String,
    pub date: String,
    pub token: String,
}

impl ParserToken {
    pub fn date(&self) -> String {
        return self.date.clone();
    }
    pub fn token(&self) -> String {
        return self.token.clone();
    }
}
