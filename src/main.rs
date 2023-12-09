use std::io::{self, Write};

use reqwest;
use scraper;

fn main() {
    let info_selector = scraper::Selector::parse("#page-content > p").unwrap();
    let mut spell_name = String::new();
    let client = reqwest::blocking::Client::new();

    loop {
        spell_name.clear();

        print!("Enter the name of the spell: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut spell_name).unwrap();
        spell_name = spell_name.replace(" ", "-");
        println!("Searching for {spell_name}...");

        let url = format!("http://dnd5e.wikidot.com/spell:{spell_name}");

        let response = client.get(&url).send().unwrap();
        assert_eq!(
            response.status(),
            reqwest::StatusCode::OK,
            "Did not find {spell_name}"
        );

        let document = scraper::Html::parse_document(&(response.text().unwrap()));

        let raw_spell_info: Vec<_> = document
            .select(&info_selector)
            .map(|element| element.text().collect::<Vec<_>>())
            .collect();
        println!(
            "Raw spell info: {:#?}",
            (&raw_spell_info[3..raw_spell_info.len() - 1])
        );
    }
}
