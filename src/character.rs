use std::collections::HashSet;

use enum_map::EnumMap;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub class: ClassLevels,
    pub attributes: EnumMap<Attribute, AttrVal>,
    pub prof_bonus: i8,
    pub skill_prof: HashSet<Skill>,
}

impl Character {
    pub fn identity(&self) -> String {
        format!("{}: {}", self.name, self.class.to_string())
    }

    pub fn create_altaea() -> Self {
        Self {
            name: "Altaea".to_string(),
            class: ClassLevels::create(
                "Artificer",
                [("Artificer", 1), ("Wizard (Order of Scribes)", 3)],
            ),
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

mod class {
    use std::collections::HashMap;

    use regex::Regex;

    #[derive(Debug, Default)]
    pub struct ClassInfo {
        subclass: Option<String>,
        levels: u8,
    }

    #[derive(Debug)]
    pub struct ClassLevels {
        pub base_class: String,
        class_levels: HashMap<String, ClassInfo>,
    }

    impl ToString for ClassLevels {
        fn to_string(&self) -> String {
            self.class_levels
                .iter()
                .map(|(key, info)| match &info.subclass {
                    Some(subclass) => format!("{} {} {}", subclass, key, info.levels),
                    None => format!("{} {}", key, info.levels),
                })
                .collect::<Vec<_>>()
                .join("/")
        }
    }

    impl ClassLevels {
        pub fn new(base_class: String) -> Self {
            let mut class_levels: HashMap<String, ClassInfo> = HashMap::new();
            class_levels.entry(base_class.clone()).or_default().levels += 1;

            Self {
                base_class,
                class_levels,
            }
        }

        pub fn create<I, T>(base_class: impl Into<String>, class_levels: I) -> Self
        where
            I: IntoIterator<Item = (T, u8)>,
            T: Into<String>,
        {
            Self {
                base_class: base_class.into(),
                class_levels: class_levels
                    .into_iter()
                    .map(|(text, levels)| {
                        let (subclass, class) = Self::split_subclass(text);
                        (class, ClassInfo { subclass, levels })
                    })
                    .collect(),
            }
        }

        fn split_subclass(text: impl Into<String>) -> (Option<String>, String) {
            let re = Regex::new(r"(.+) \((.+)\)").unwrap();
            let text: String = text.into();

            match re.captures(&text) {
                None => (None, text),
                Some(caps) => (Some(caps[2].into()), caps[1].into()),
            }
        }

        pub fn add_level(mut self, class: String) -> Self {
            self.class_levels.entry(class).or_default().levels += 1;
            self
        }

        pub fn modify_subclass(mut self, class: String, subclass: String) -> Self {
            self.class_levels
                .entry(class)
                .and_modify(|info| info.subclass = Some(subclass));
            self
        }
    }
}
use class::ClassLevels;
