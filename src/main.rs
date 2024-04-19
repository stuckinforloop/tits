use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tits_rs::app::{App, AppResult};
use tits_rs::event::{Event, EventHandler};
use tits_rs::handler::handle_key_events;
use tits_rs::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new();
    let _sender = events.sender.clone();
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Create an application.
    let mut app = App::new(_sender);

    // Start the main loop.
    while !app.exit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
