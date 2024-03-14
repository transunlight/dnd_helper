use enum_map::EnumMap;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub attributes: EnumMap<Attribute, AttrVal>,
    pub prof_bonus: i8,
}

impl Character {
    pub fn create_altaea() -> Self {
        Self {
            name: "Altaea".to_string(),
            attributes: EnumMap::default(),
            prof_bonus: 2,
        }
    }
}

mod attribute {
    use enum_map::Enum;
    use strum::Display;

    #[derive(Debug, Enum, Display)]
    pub enum Attribute {
        Strength,
        Dexterity,
        Constitution,
        Intelligence,
        Wisdom,
        Charisma,
    }

    #[derive(Debug, Default)]
    pub struct AttrVal {
        pub score: i8,
    }

    impl AttrVal {
        pub fn modifier(&self) -> i8 {
            (self.score - 10) / 2
        }
    }

    impl From<i8> for AttrVal {
        fn from(score: i8) -> Self {
            Self { score }
        }
    }
}
use attribute::{AttrVal, Attribute};
