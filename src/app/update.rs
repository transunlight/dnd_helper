use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};
use log::warn;

pub enum Action {
    Quit,
    None,
}

pub fn update(key_event: KeyEvent) -> Action {
    match key_event.code {
        Char('c') | Char('C') if key_event.modifiers == KeyModifiers::CONTROL => Action::Quit,
        Char('q') | KeyCode::Esc => Action::Quit,
        Char('p') => panic!("pressed `p`"),
        l => {
            warn!("not implemented: {l:?}");
            Action::None
        }
    }
}
