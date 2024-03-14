use log::warn;

use crate::{character::Character, tui::Event};

pub mod update;
use update::Action;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub current_screen: CurrentScreen,
    pub current_character: Option<Character>,
}

impl App {
    pub fn new() -> Self {
        Self::default().load_character()
    }

    pub fn load_character(mut self) -> Self {
        self.current_character = Some(Character::create_altaea());
        self
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::Tick => self.tick(),
            Event::Key(key_event) => match update::update(key_event) {
                Action::Quit => self.quit(),
                Action::None => (),
            },
            event => warn!("not implemented: {event:?}"),
        }
    }
}
