// Headifier 0.4.0
// David Serrano
// January 3rd, 2023


use crate::ui::app::App;
use crate::ui::event::{Event, EventHandler};
use crate::ui::tui::Tui;
use crate::ui::update::{update, update_screen};
use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

pub fn init_ratui() -> Result<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(width, height) => update_screen(&mut app, width, height)
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}





