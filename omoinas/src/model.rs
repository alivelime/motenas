pub mod mono;

#[derive(Clone, Debug)]
pub struct Nani {
    pub donna: Option<String>,
    pub mono: Vec<String>,
}
impl Nani {
    pub fn donna_namae(&self) -> String {
        return format!(
            "{}{}",
            self.donna.as_ref().unwrap_or(&String::from("")),
            self.namae()
        );
    }
    pub fn namae(&self) -> String {
        return self
            .mono
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<String>>()
            .join("");
    }
    pub fn has(&self, w: Vec<&str>) -> bool {
        return self.mono.iter().any(|m| w.contains(&m.as_str()));
    }
}
