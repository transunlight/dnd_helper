use std::collections::HashSet;

use enum_map::EnumMap;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub attributes: EnumMap<Attribute, AttrVal>,
    pub prof_bonus: i8,
    pub skill_prof: HashSet<Skill>,
}

impl Character {
    pub fn create_altaea() -> Self {
        Self {
            name: "Altaea".to_string(),
            attributes: EnumMap::default(),
            prof_bonus: 2,
            skill_prof: HashSet::default(),
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

mod skill {
    use strum::Display;

    #[derive(Debug, Display)]
    pub enum Skill {
        Acrobatics,
        Athletics,
        SleightOfHand,
        Stealth,
        Arcana,
        History,
        Investigation,
        Nature,
        Religion,
        AnimalHandling,
        Insight,
        Medicine,
        Perception,
        Survival,
        Deception,
        Intimidation,
        Performance,
        Persuasion,
    }

    #[allow(dead_code)]
    pub enum SkillLevel {
        Untrained,
        Proficient,
        Expert,
    }
}
use skill::Skill;
