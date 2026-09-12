use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use std::time::Duration;

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    pub fn next(&self) -> Result<AppEvent> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => Ok(AppEvent::Key(key)),
                _ => Ok(AppEvent::Tick),
            }
        } else {
            Ok(AppEvent::Tick)
        }
    }
}
