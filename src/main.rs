mod app;
mod config;
mod event;
mod package;
mod pacman;
mod ui;

use app::App;
use config::load_config;
use event::{AppEvent, EventHandler};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Load config
    let config = load_config();

    // Initialize terminal
    let mut terminal = ratatui::init();
    terminal.clear()?;

    // Create app
    let mut app = App::new(config)?;

    // Create event handler
    let events = EventHandler::new(Duration::from_millis(250));

    // Main loop
    while !app.should_quit {
        // Render
        terminal.draw(|frame| {
            ui::render(frame, &mut app);
        })?;

        // Handle events
        match events.next()? {
            AppEvent::Key(key) => app.handle_key(key),
            AppEvent::Tick => {}
        }
    }

    // Restore terminal
    ratatui::restore();

    Ok(())
}
