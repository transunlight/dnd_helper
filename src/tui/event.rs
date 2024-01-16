use std::time::Duration;

use color_eyre::eyre::{eyre, Result};
use crossterm::event::{EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

mod event_def {
    use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};

    #[derive(Debug)]
    pub enum Event {
        Tick,
        Mouse(MouseEvent),
        Key(KeyEvent),
        Resize(u16, u16),
        FocusLost,
        FocusGained,
        Paste(String),
    }

    impl From<CrosstermEvent> for Event {
        fn from(event: CrosstermEvent) -> Self {
            match event {
                CrosstermEvent::FocusLost => Self::FocusLost,
                CrosstermEvent::FocusGained => Self::FocusGained,
                CrosstermEvent::Key(e) => Self::Key(e),
                CrosstermEvent::Mouse(e) => Self::Mouse(e),
                CrosstermEvent::Resize(w, h) => Self::Resize(w, h),
                CrosstermEvent::Paste(string) => Self::Paste(string),
            }
        }
    }
}
pub use event_def::Event;

pub struct EventHandler {
    #[allow(dead_code)]
    sender: UnboundedSender<Event>,
    receiver: UnboundedReceiver<Event>,
    #[allow(dead_code)]
    handle: Option<JoinHandle<()>>,
}

impl EventHandler {
    pub fn new(tick_rate: u8) -> Self {
        let tick_interval = Duration::from_secs_f64(1.0 / tick_rate as f64);

        let (sender, receiver) = unbounded_channel();

        let handle = {
            let sender = sender.clone();
            tokio::spawn(async move {
                let mut reader = EventStream::new();
                let mut tick = tokio::time::interval(tick_interval);

                loop {
                    let tick_delay = tick.tick();
                    let crossterm_event = reader.next().fuse();

                    tokio::select! {
                        maybe_event = crossterm_event => {
                            let event = maybe_event
                                .expect("no event received")
                                .expect("failed to read terminal event");
                            match Event::from(event) {
                                Event::Key(e) if e.kind != KeyEventKind::Press => Ok(()),
                                event => sender.send(event),
                            }.expect("failed to send terminal event");
                        },
                        _ = tick_delay => {
                            sender.send(Event::Tick).expect("failed to send tick event");
                        }
                    };
                }
            })
        };

        Self {
            sender,
            receiver,
            handle: Some(handle),
        }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(eyre!("unable to get event"))
    }
}
