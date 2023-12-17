use std::io::{self, Write};

use scraper::Selector;

fn main() {
    let info_selector = Selector::parse("#page-content > *").unwrap();
    let client = reqwest::blocking::Client::new();

    while let Some(spell_name) = get_spell_name() {
        println!("Searching for {spell_name}...");

        let url = format!("http://dnd5e.wikidot.com/spell:{spell_name}");

        let response = match client.get(&url).send() {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        if response.status() != reqwest::StatusCode::OK {
            println!("Could not find {spell_name}.");
            continue;
        }

        let document = scraper::Html::parse_document(&(response.text().unwrap()));

        let raw_spell_info: Vec<_> = document
            .select(&info_selector)
            .flat_map(|element| element.text())
            .map(|text| text.trim())
            .filter(|text| !text.is_empty())
            .collect();

        println!("Raw spell info: {:#?}", (&raw_spell_info));
    }

    println!("Closing...");
}

fn get_spell_name() -> Option<String> {
    let mut name = String::new();

    print!("Enter the name of the spell: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().replace(' ', "-");

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
