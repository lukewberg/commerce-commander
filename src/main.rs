use commerce_commander::core::{
    app::App,
    event::{Event, EventHandler},
    init::{Err, Result},
    tui::Tui,
    update::update,
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new(
        String::from("Keep it secret, keep it safe!"),
        String::from("Hello world!"),
    );

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
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
    // startup()?;
    // run()?;
    // shutdown()?;
    // Ok(())
}
