#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub prof_bonus: i8,
}

impl Character {
    pub fn create_altaea() -> Self {
        Self {
            name: "Altaea".to_string(),
            prof_bonus: 2,
        }
    }
}
