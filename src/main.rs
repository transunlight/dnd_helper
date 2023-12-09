use std::io::{self, Write};

fn main() {
    let info_selector = scraper::Selector::parse("#page-content > p").unwrap();
    let client = reqwest::blocking::Client::new();

    loop {
        let spell_name = match get_spell_name() {
            Some(name) => name,
            None => break,
        };

        println!("Searching for {spell_name}...");

        let url = format!("http://dnd5e.wikidot.com/spell:{spell_name}");

        let response = client.get(&url).send().unwrap();
        if response.status() != reqwest::StatusCode::OK {
            println!("Could not find {spell_name}.");
            continue;
        }

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

fn get_spell_name() -> Option<String> {
    let mut name = String::new();

    print!("Enter the name of the spell: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().replace(" ", "-");

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
