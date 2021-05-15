mod app;
use crate::app::App;

use crossterm::{
    event,
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // Input handling thread
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        // The timeout doesn't seem to affect anything,
        // but it consumes a lot of cpu if set to 0 and looped
        if event::poll(Duration::from_secs(10)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                tx.send(key).unwrap();
            }
        }
    });

    // Initialize application
    let mut app = App::new();

    // Main drawing loop
    loop {
        // Draw UI
        app.draw(&mut terminal);

        // Input handling
        // For whatever reason, this MUST be after the drawing
        // I believe it has to do with the fact that this is async
        match rx.recv()?.code {
            KeyCode::Char('q') => break,
            KeyCode::Enter => app.enter(),
            KeyCode::Char('h') => app.left(),
            KeyCode::Char('j') => app.down(),
            KeyCode::Char('k') => app.up(),
            KeyCode::Char('l') => app.right(),
            _ => {}
        }
    }

    // Cleanup and exit
    app.exit();
    disable_raw_mode()?;
    terminal.show_cursor()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
