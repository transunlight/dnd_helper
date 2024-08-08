use std::{io, panic};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;

mod event;
mod ui;

pub use event::Event;

pub type CrosstermTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct Tui {
    terminal: CrosstermTerminal,
    pub events: event::EventHandler,
}

impl Tui {
    pub fn new(tick_rate: u8) -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout()))?,
            events: event::EventHandler::new(tick_rate),
        })
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            if let Err(err) = Self::reset() {
                eprintln!("{err}")
            };
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(frame, app))?;
        Ok(())
    }

    fn reset() -> Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
