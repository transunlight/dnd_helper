// use std::collections::HashSet;

use std::str::FromStr;

use strum::EnumString;

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Class {
    Artificer,
    Bard,
    Cleric,
    Druid,
    Ranger,
    Paladin,
    Sorcerer,
    Warlock,
    Wizard,
}

#[derive(Debug)]
pub enum SpellLevel {
    Cantrip,
    Level(u8),
}

impl SpellLevel {
    // level currently simply returns the first character as a digit as the level
    fn level(s: &str) -> Option<u8> {
        match s.split_once('-') {
            Some((s, "level")) => s
                .chars()
                .next()
                .and_then(|d| d.to_digit(10))
                .map(|d| d as u8),
            _ => None,
        }
    }
}

impl FromStr for SpellLevel {
    type Err = ();
    // Improve error handling

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cantrip" => Ok(SpellLevel::Cantrip),
            s => match SpellLevel::level(s) {
                Some(level) => Ok(SpellLevel::Level(level)),
                None => Err(()),
            },
        }
    }
}

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

pub enum Components {
    Verbal,
    Somatic,
    Material(String),
}

#[derive(Debug)]
pub struct Spell {
    pub level: SpellLevel,
    pub name: String,
    pub description: String,
    pub school: SpellSchool,
    pub casting_time: String,
    pub range: String,
    // pub concentration: bool,
    // pub components: HashSet<Components>,
    pub duration: String,
    // pub lists: HashSet<Class>,
    pub source: String,
}

impl Spell {
    // Improve error handling
    pub fn from_raw_vector(name: String, raw_info: Vec<&str>) -> Spell {
        let source = match raw_info[0].split_once(": ") {
            Some(("Source", source)) => source.to_string(),
            s => panic!("Incorrect input: {:?}", s),
        };

        let (level, school) = match raw_info[1].split_once(' ') {
            Some((school, level @ "cantrip")) | Some((level, school)) => (
                SpellLevel::from_str(level).unwrap(),
                SpellSchool::from_str(school).unwrap(),
            ),
            s => panic!("Incorrect input: {:?}", s),
        };

        let casting_time = match (raw_info[2], raw_info[3]) {
            ("Casting Time:", s) => s.to_string(),
            s => panic!("Incorrect input {:?}", s),
        };

        let range = match (raw_info[4], raw_info[5]) {
            ("Range:", s) => s.to_string(),
            s => panic!("Incorrect input {:?}", s),
        };

        // let components = match (raw_info[6], raw_info[7]) {
        //     ("Components:", s) => s.to_string(),
        //     s => panic!("Incorrect input {:?}", s),
        // };

        let duration = match (raw_info[8], raw_info[9]) {
            ("Duration:", s) => s.to_string(),
            s => panic!("Incorrect input {:?}", s),
        };

        let description = raw_info[10..raw_info.len()].join("\n");

        Spell {
            name,
            description,
            school,
            level,
            source,
            casting_time,
            range,
            duration,
        }
    }
}
