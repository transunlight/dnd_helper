use std::io::{self, Write};

use reqwest::blocking::{Client, Response};
use scraper::Html;

use dnd_helper::Spell;

fn main() {
    let client = Client::new();

    while let Some(spell_name) = get_spell_name() {
        println!("Searching for {spell_name}...");

        let response = match get_spell_info(&client, &spell_name) {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        let document = Html::parse_document(response.text().unwrap().as_str());
        let spell = Spell::from_spell_document(spell_name, document);

        println!("Spell struct: {:#?}", spell);
    }

    println!("Closing...");
}

fn get_spell_name() -> Option<String> {
    let mut name = String::new();

    print!("Enter the name of the spell: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn get_spell_info(client: &Client, spell_name: &str) -> Result<Response, reqwest::Error> {
    let spell_name = spell_name.replace(' ', "-");
    let url = format!("http://dnd5e.wikidot.com/spell:{spell_name}");

    client
        .get(url)
        .send()
        .and_then(|res| res.error_for_status())
}
