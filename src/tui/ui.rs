use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Row, Table};

use crate::app::App;
use crate::character::Character;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    render_header(frame, chunks[0], &app.current_character);
    render_main(frame, chunks[1], app);
    render_footer(frame, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, maybe_character: &Option<Character>) {
    let header = Paragraph::new(Line::styled(
        match maybe_character {
            None => "No character chosen".into(),
            Some(character) => character.identity(),
        },
        Style::default().fg(Color::Red),
    ))
    .block(Block::bordered());

    f.render_widget(header, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let commands = [("q", "Exit"), ("p", "Panic")];

    let spans: Vec<_> = commands
        .iter()
        .map(|(c, mean)| Span::raw(format!("{} {} ", c, mean)))
        .collect();

    let footer = Paragraph::new(Line::from(spans)).block(Block::bordered());

    f.render_widget(footer, area);
}

fn render_main(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::horizontal([Constraint::Percentage(25), Constraint::Min(1)]).split(area);

    let chunks_0 = Layout::vertical([
        Constraint::Length(9),
        Constraint::Percentage(50),
        Constraint::Min(1),
    ])
    .split(chunks[0]);

    if let Some(character) = &app.current_character {
        render_attributes(f, chunks_0[0], character);
        render_proficiencies(f, chunks_0[1], character);
    }
}

fn render_attributes(f: &mut Frame, area: Rect, character: &Character) {
    let header = Row::new(vec!["Attribute", "Score", "Mod"]).bold();
    let widths = [15, 5, 3];

    let rows = character.attributes.iter().map(|(attr, val)| {
        Row::new([
            attr.to_string(),
            format!("{:^5}", val.score),
            format!("{:^+3}", val.modifier()),
        ])
    });

    let attributes = Table::new(rows, widths)
        .header(header)
        .block(Block::bordered().title("Attributes"));

    f.render_widget(attributes, area);
}

fn render_proficiencies(f: &mut Frame, area: Rect, character: &Character) {
    let header = Row::new(vec!["Skill", "Bonus"]).bold();
    let widths = [15, 5];

    let rows = character
        .skill_prof
        .get_proficient()
        .map(|skill| Row::new(vec![skill.to_string(), character.prof_bonus.to_string()]));

    let attributes = Table::new(rows, widths)
        .header(header)
        .block(Block::bordered().title("Proficiencies"));

    f.render_widget(attributes, area);
}
