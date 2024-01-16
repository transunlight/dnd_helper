use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    render_header(frame, chunks[0]);
    render_main(frame, chunks[1]);
    render_footer(frame, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(Line::styled(
        "Altaea: Artificer 1/Order of Scribes Wizard 3",
        Style::default().fg(Color::Red),
    ))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let commands = [("q", "Exit"), ("p", "Panic")];

    let spans: Vec<_> = commands
        .iter()
        .map(|(c, mean)| Span::raw(format!("{} {} ", c, mean)))
        .collect();

    let footer = Paragraph::new(Line::from(spans)).block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}

fn render_main(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Min(1)])
        .split(area);

    let chunks_0 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Percentage(50),
            Constraint::Min(1),
        ])
        .split(chunks[0]);

    render_attributes(f, chunks_0[0]);
    render_proficiencies(f, chunks_0[1]);
}

fn render_attributes(f: &mut Frame, area: Rect) {
    let header = Row::new(vec!["Attribute", "Score", "Save"]).bold();
    let rows = [
        Row::new(vec!["Strength", "8", "-1"]),
        Row::new(vec!["Dexterity", "14", "+2"]),
        Row::new(vec!["Constitution", "18", "+6"]),
        Row::new(vec!["Intelligence", "20", "+7"]),
        Row::new(vec!["Wisdom", "14", "+2"]),
        Row::new(vec!["Charisma", "8", "-1"]),
    ];
    let widths = [
        Constraint::Length(15),
        Constraint::Length(5),
        Constraint::Length(5),
    ];

    let attributes = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Attributes"));

    f.render_widget(attributes, area);
}

fn render_proficiencies(f: &mut Frame, area: Rect) {
    let header = Row::new(vec!["Skill", "Bonus"]).bold();
    let rows = [
        Row::new(vec!["Arcana", "+2"]),
        Row::new(vec!["History", "+2"]),
        Row::new(vec!["Investigation", "+2"]),
        Row::new(vec!["Perception", "+2"]),
        Row::new(vec!["Insight", "+2"]),
    ];
    let widths = [Constraint::Length(15), Constraint::Length(5)];

    let attributes = Table::new(rows, widths).header(header).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Proficiencies"),
    );

    f.render_widget(attributes, area);
}
