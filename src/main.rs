use std::io::{self, Write};

use reqwest::blocking::{Client, Response};
use scraper::Selector;

fn main() {
    let info_selector = Selector::parse("#page-content > *").unwrap();
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

        let document = scraper::Html::parse_document(response.text().unwrap().as_str());

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
