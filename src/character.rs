#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub class: ClassLevels,
    pub attributes: Attributes,
    pub prof_bonus: i8,
    pub skill_prof: SkillProficiencies,
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
            attributes: Attributes::new([8, 14, 18, 20, 16, 14]),
            prof_bonus: 2,
            skill_prof: SkillProficiencies::default(),
        }
    }
}

mod attribute {
    use enum_map::{enum_map, Enum};
    use strum::Display;

    use crate::utils::MyEnumMap;

    pub type Attributes = MyEnumMap<Attribute, AttrVal>;

    impl Attributes {
        pub fn new(scores: [i8; 6]) -> Self {
            enum_map! {
                Attribute::Strength => scores[0].into(),
                Attribute::Dexterity => scores[1].into(),
                Attribute::Constitution => scores[2].into(),
                Attribute::Intelligence => scores[3].into(),
                Attribute::Wisdom => scores[4].into(),
                Attribute::Charisma => scores[5].into(),
            }
            .into()
        }
    }

    #[derive(Debug, Enum, Display)]
    pub enum Attribute {
        Strength,
        Dexterity,
        Constitution,
        Intelligence,
        Wisdom,
        Charisma,
    }

    #[derive(Debug)]
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
use attribute::Attributes;

mod skill {
    use enum_map::Enum;
    use strum::Display;

    use crate::utils::MyEnumMap;

    #[derive(Debug, Display, Enum, PartialEq, Eq, Hash)]
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

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
    pub enum SkillLevel {
        #[default]
        Untrained,
        Proficient,
        Expert,
    }

    pub type SkillProficiencies = MyEnumMap<Skill, SkillLevel>;

    impl<'a> SkillProficiencies {
        pub fn get_proficient(&'a self) -> impl Iterator<Item = Skill> + 'a {
            self.get(SkillLevel::Proficient)
        }

        pub fn get_expert(&'a self) -> impl Iterator<Item = Skill> + 'a {
            self.get(SkillLevel::Expert)
        }

        fn get(&'a self, level_filter: SkillLevel) -> impl Iterator<Item = Skill> + 'a {
            self.iter()
                .filter_map(move |(skill, &level)| (level == level_filter).then_some(skill))
        }
    }
}
use skill::SkillProficiencies;

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
